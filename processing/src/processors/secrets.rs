use crate::data::stacks::StackDocument;
use crate::processors::processor::Processor;
use crate::yaml::{MappingExt, YamlOwnedExt};
use async_trait::async_trait;
use log::debug;
use saphyr::{ScalarOwned, YamlOwned};
use shared::data::Config;
use std::env;
use std::fs;
use std::path::PathBuf;

pub struct SecretProcessor {
    secrets_dir: PathBuf,
}

impl SecretProcessor {
    pub fn new() -> Self {
        Self {
            secrets_dir: PathBuf::new(),
        }
    }
}

#[async_trait]
impl Processor for SecretProcessor {
    async fn setup(&mut self, config: &Config) -> Result<(), String> {
        let secrets_dir = config.paths.out.join("secrets");
        fs::create_dir_all(&secrets_dir)
            .map_err(|e| format!("Failed to create secrets directory: {}", e))?;

        self.secrets_dir = secrets_dir;

        Ok(())
    }

    async fn process(&self, doc: &mut StackDocument, config: &Config) -> Result<(), String> {
        debug!("{}: Processing secrets...", doc.stack_name);
        process_secrets(doc, config, self).await?;

        Ok(())
    }
}

pub async fn process_secrets(
    doc: &mut StackDocument,
    config: &Config,
    context: &SecretProcessor,
) -> Result<(), String> {
    let yaml = &mut doc.root;

    let Some(secrets) = yaml
        .get_value_mut("secrets")
        .map(YamlOwned::as_mapping_mut)
        .flatten()
    else {
        return Ok(());
    };

    let secrets_to_process: Vec<(String, String)> = secrets
        .iter()
        .filter_map(|(secret_name, secret_value)| {
            let secret_name = secret_name.as_str()?.to_string();
            let secret_entry = secret_value.as_mapping()?;
            let env_var_name = secret_entry.get_value("environment")?.as_str()?.to_string();
            Some((secret_name, env_var_name))
        })
        .collect();

    for (secret_name, env_var_name) in secrets_to_process {
        let env_value = env::var(&env_var_name)
            .map_err(|_| format!("Environment variable '{}' not found", env_var_name))?;

        let secret_file_path = context.secrets_dir.join(&secret_name);
        fs::write(&secret_file_path, &env_value)
            .map_err(|e| format!("Failed to write secret file: {}", e))?;

        if let Some(secret_entry) = secrets
            .get_mut(&YamlOwned::value_of(secret_name))
            .and_then(YamlOwned::as_mapping_mut)
        {
            secret_entry.remove(&YamlOwned::value_of("environment"));
            secret_entry.insert(
                YamlOwned::value_of("file"),
                YamlOwned::value_of(secret_file_path.to_string_lossy()),
            );
        }
    }

    Ok(())
}
