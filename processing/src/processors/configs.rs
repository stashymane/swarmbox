use crate::data::stacks::StackDocument;
use crate::processors::processor::Processor;
use crate::yaml::{MappingExt, YamlOwnedExt};
use async_trait::async_trait;
use log::{debug, trace};
use saphyr::{MappingOwned, YamlOwned};
use sha2::{Digest, Sha256};
use shared::data::{Config, RelativePath};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::read;
use util::walk_path;

pub struct ConfigProcessor {
    configs: HashMap<String, ConfigEntry>,
}

impl ConfigProcessor {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }
}

pub struct ConfigEntry {
    name: String,
    absolute_path: PathBuf,
    project_path: RelativePath,
}

#[async_trait]
impl Processor for ConfigProcessor {
    async fn setup(&mut self, config: &Config) -> Result<(), String> {
        let config_dir = &config.paths.configs;
        let config_files = walk_path(config_dir)
            .await
            .map_err(|e| format!("Failed to walk config: {:?}", e))?;

        for path in config_files {
            let project_path = RelativePath::from(&path, config_dir)
                .map_err(|e| format!("Failed to retrieve relative path: {:?}", e))?;
            let name = Option::ok_or(project_path.name(), "Failed to retrieve project name")?;

            let entry = ConfigEntry {
                name: name.to_owned(),
                absolute_path: path,
                project_path,
            };

            self.configs.insert(name, entry);
        }

        Ok(())
    }

    async fn process(&self, doc: &mut StackDocument, _config: &Config) -> Result<(), String> {
        debug!("{}: Processing configs...", doc.stack_name);
        let resolved_configs = collect_and_rewrite_configs(self, &mut doc.root).await?;
        insert_top_level_configs(&mut doc.root, &resolved_configs);
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct ResolvedConfig {
    key: String,
    name: String,
    file: String,
}

async fn collect_and_rewrite_configs(
    context: &ConfigProcessor,
    yaml: &mut MappingOwned,
) -> Result<HashMap<String, ResolvedConfig>, String> {
    let Some(services) = yaml
        .get_value_mut("services")
        .map(YamlOwned::as_mapping_mut)
        .flatten()
    else {
        return Ok(HashMap::new());
    };

    let mut resolved_configs: HashMap<String, ResolvedConfig> = HashMap::new();

    for (key, service) in services.iter_mut() {
        let Some(service) = service.as_mapping_mut() else {
            trace!("Service {:?} is not a mapping, skipping", key.as_str());
            continue;
        };

        let Some(configs) = service
            .get_value_mut("configs")
            .map(YamlOwned::as_sequence_mut)
            .flatten()
        else {
            trace!("Service {:?} has no configs, skipping", key.as_str());
            continue;
        };

        for config_element in configs.iter_mut() {
            let Some(config_map) = config_element.as_mapping_mut() else {
                trace!(
                    "Config {:?} is not a mapping, skipping",
                    config_element.as_str()
                );
                continue;
            };

            let Some(YamlOwned::Tagged(tag, source)) =
                config_map.get(&YamlOwned::value_of("source"))
            else {
                continue;
            };

            if tag.suffix != "config" {
                continue;
            }

            let Some(config_name) = source.as_str().map(|it| it.to_string()) else {
                continue;
            };

            // If already resolved, just update the source and skip re-resolving
            if let Some(resolved) = resolved_configs.get(&config_name) {
                config_map.insert(
                    YamlOwned::value_of("source"),
                    YamlOwned::value_of(resolved.key.clone()),
                );
                continue;
            }

            let Some(config) = context.configs.get(&config_name) else {
                return Err(format!("Config {} not found", config_name));
            };
            let full_path = &config.absolute_path;

            let key = Option::ok_or_else(safe_config_name(&config.project_path), || {
                format!("Config {} has an invalid path name", config_name)
            })?;
            let name =
                Option::ok_or_else(hashed_config_name(&key, full_path.as_path()).await, || {
                    format!("Failed to hash config {}", config_name)
                })?;

            config_map.insert(
                YamlOwned::value_of("source"),
                YamlOwned::value_of(key.clone()),
            );

            resolved_configs.insert(
                config_name.to_string(),
                ResolvedConfig {
                    key,
                    name,
                    file: full_path.to_string_lossy().into_owned(),
                },
            );
        }
    }

    Ok(resolved_configs)
}

fn insert_top_level_configs(
    yaml: &mut MappingOwned,
    resolved_configs: &HashMap<String, ResolvedConfig>,
) {
    if resolved_configs.is_empty() {
        return;
    }

    let mut entries = resolved_configs.values().cloned().collect::<Vec<_>>();
    entries.sort_by(|a, b| a.key.cmp(&b.key));

    let mut mapping = MappingOwned::new();

    for resolved in entries {
        let mut content = MappingOwned::new();
        content.insert(
            YamlOwned::value_of("name"),
            YamlOwned::value_of(resolved.name),
        );
        content.insert(
            YamlOwned::value_of("file"),
            YamlOwned::value_of(resolved.file),
        );

        mapping.insert(
            YamlOwned::value_of(resolved.key),
            YamlOwned::Mapping(content),
        );
    }

    yaml.insert(YamlOwned::value_of("configs"), YamlOwned::Mapping(mapping));
}

pub fn safe_config_name(path: &RelativePath) -> Option<String> {
    Some(path.name()?.replace("/", "__"))
}

async fn hashed_config_name(base: &str, full_path: &std::path::Path) -> Option<String> {
    let bytes = read(full_path).await.ok()?;

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();

    let short_hash = format!("{:x}", digest);
    Some(format!("{}-{}", base, &short_hash[..12]))
}
