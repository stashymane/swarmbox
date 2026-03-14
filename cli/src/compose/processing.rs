use crate::compose::context::{Context, ProjectPath};
use crate::compose::processors::configs::process_configs;
use crate::compose::processors::includes::merge_includes;
use crate::compose::stacks::{StackDocument, StackDocumentError};
use crate::compose::yaml::write_yml;
use saphyr::YamlOwned;

type Processor = fn(&mut StackDocument, &Context) -> Result<(), String>;

pub fn generate_stack(context: &Context, stack: &String) -> Result<String, String> {
    println!("Generating stack {:?}...", stack);
    let processors: [Processor; 2] = [merge_includes, process_configs];

    let Some((name, project_path)) = find_stack_by_name(context, stack) else {
        return Err(format!("Could not find {}.yml", stack));
    };

    let mut doc = match StackDocument::load(name, context) {
        Ok(doc) => doc,
        Err(StackDocumentError::Invalid) => return Err("Failed to load stack".to_string()),
        Err(StackDocumentError::NotFound) => return Err("Stack not found".to_string()),
    };

    for processor in processors {
        processor(&mut doc, context)?;
    }

    write_yml(&YamlOwned::Mapping(doc.root), &doc.output_path);

    Ok(name.to_string())
}

fn find_stack_by_name<'a>(
    state: &'a Context,
    stack_name: &String,
) -> Option<(&'a String, &'a ProjectPath)> {
    state
        .sources
        .iter()
        .filter(|(name, _)| {
            *name == stack_name
                || name
                    .strip_suffix(".yml")
                    .or_else(|| name.strip_suffix(".yaml"))
                    .map(|s| s == stack_name)
                    .unwrap_or(false)
        })
        .next()
}
