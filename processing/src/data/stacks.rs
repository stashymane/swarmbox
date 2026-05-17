use crate::yaml::{read_yml, write_yml};
use saphyr::{MappingOwned, Yaml, YamlOwned};
use shared::data::{Config, RelativePath};
use std::fmt::Debug;
use std::path::PathBuf;
use thiserror::Error;

pub struct StackDocument {
    pub stack_name: String,
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub root: MappingOwned,
}

#[derive(Error, Debug)]
pub enum StackDocumentError {
    #[error("Stack \"{0}\" not found")]
    NotFound(String),
    #[error("Stack \"{0}\" is not valid: {1}")]
    Invalid(String, String),
    #[error("Stack name \"{0}\" is invalid")]
    InvalidName(String),
}

impl StackDocument {
    pub async fn load(
        project_path: &RelativePath,
        config: &Config,
    ) -> Result<StackDocument, StackDocumentError> {
        let source_path = project_path.get_absolute_path(&config.paths.source);
        let name = Option::ok_or_else(project_path.name(), || {
            StackDocumentError::InvalidName(project_path.to_str().unwrap().to_owned())
        })?;

        let mut yaml_vec = read_yml(&source_path).await;

        let yaml = Option::ok_or_else(yaml_vec.pop(), || {
            StackDocumentError::NotFound(name.clone())
        })?;

        let mapping = match yaml {
            YamlOwned::Mapping(mapping) => mapping,
            _ => {
                return Err(StackDocumentError::Invalid(
                    name.clone(),
                    "Expected root to be a mapping".to_string(),
                ));
            }
        };

        let output_path = project_path.get_absolute_path(&config.paths.out);

        let doc = StackDocument {
            stack_name: name.to_owned(),
            source_path,
            output_path: output_path.clone(),
            root: mapping,
        };

        Ok(doc)
    }

    pub async fn write(self) -> PathBuf {
        let yaml = YamlOwned::Mapping(self.root);
        write_yml(&Yaml::from(&yaml), &self.output_path).await;
        self.output_path
    }
}
