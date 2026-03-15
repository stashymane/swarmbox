use crate::data::context::ProcessingContext;
use crate::data::stacks::StackDocument;
use crate::processors::processor::Processor;
use crate::yaml::{MappingExt, YamlOwnedExt};
use async_trait::async_trait;
use log::debug;
use saphyr::{MappingOwned, YamlOwned};
use shared::data::{Config, RelativePath};
use std::collections::HashMap;
use util::walk_path;

pub struct IncludeProcessor {
    sources: HashMap<String, SourceEntry>,
}

impl IncludeProcessor {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }
}

struct SourceEntry {
    name: String,
    path: RelativePath,
}

#[async_trait]
impl Processor for IncludeProcessor {
    async fn setup(&mut self, config: &Config) -> Result<(), String> {
        let source_dir = &config.paths.source;
        let source_paths = walk_path(source_dir)
            .await
            .map_err(|e| format!("Failed to walk source: {:?}", e))?;

        for path in source_paths {
            if path.extension() == Some("yml".as_ref()) || path.extension() == Some("yaml".as_ref())
            {
                let project_path = RelativePath::from(&path, source_dir)
                    .map_err(|e| format!("Failed to retrieve relative path: {:?}", e))?;
                let name = project_path
                    .name()
                    .ok_or("Failed to retrieve project name")?;

                let entry = SourceEntry {
                    name: name.to_owned(),
                    path: project_path,
                };

                self.sources.insert(name, entry);
            }
        }

        Ok(())
    }

    async fn process(&self, doc: &mut StackDocument, config: &Config) -> Result<(), String> {
        debug!("{}: Processing includes...", doc.stack_name);
        merge_includes_recursive(config, self, &mut doc.root, &mut Vec::new()).await?;
        Ok(())
    }
}

async fn merge_includes_recursive(
    config: &Config,
    processor: &IncludeProcessor,
    yaml: &mut MappingOwned,
    stack: &mut Vec<String>,
) -> Result<(), String> {
    let key = YamlOwned::value_of("include");

    let Some(include) = yaml.get(&key) else {
        debug!("Include section not found, continuing...");
        return Ok(());
    };
    let Some(contents) = include.as_sequence() else {
        debug!("Include is not a sequence, continuing...");
        return Ok(());
    };

    let includes = contents
        .iter()
        .filter_map(|yaml| yaml.as_str())
        .map(|str| str.to_owned())
        .map(|name| {
            processor
                .sources
                .get(&name)
                .ok_or_else(|| format!("Include not found: {:?}", name))
        })
        .collect::<Result<Vec<_>, String>>()?;

    yaml.remove(&key);

    for entry in includes.iter() {
        if stack.contains(&entry.name) {
            return Err(format!("Circular include detected: {:?}", entry.name));
        }

        debug!("Merging module {:?}", entry.name);
        let mut doc = StackDocument::load(&entry.path, config)
            .await
            .map_err(|e| format!("Failed to merge include: {:?}", e))?;

        stack.push(entry.name.to_owned());
        Box::pin(merge_includes_recursive(
            config,
            processor,
            &mut doc.root,
            stack,
        ))
        .await?;

        yaml.merge_from(&doc.root);
    }

    Ok(())
}
