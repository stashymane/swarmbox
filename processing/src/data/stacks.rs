use crate::yaml::{read_yml, write_yml};
use saphyr::{MappingOwned, Yaml, YamlOwned};
use shared::data::{Config, RelativePath};
use std::fmt::Debug;
use std::path::PathBuf;

pub struct StackDocument {
    pub stack_name: String,
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub root: MappingOwned,
}

#[derive(Debug)]
pub enum StackDocumentError {
    NotFound(String),
    Invalid(String),
}

impl StackDocumentError {
    fn invalid_name(name: impl Debug) -> Self {
        StackDocumentError::Invalid(format!("Stack name is invalid: {:?}", name))
    }

    fn not_found(name: impl Debug) -> Self {
        StackDocumentError::NotFound(format!("Stack \"{:?}\" not found", name))
    }

    fn not_valid(name: impl Debug) -> Self {
        StackDocumentError::Invalid(format!("Stack \"{:?}\" is not valid", name))
    }
}

impl StackDocument {
    pub async fn load(
        project_path: &RelativePath,
        config: &Config,
    ) -> Result<StackDocument, StackDocumentError> {
        let source_path = project_path.get_absolute_path(&config.paths.source);
        let name = Option::ok_or_else(project_path.name(), || {
            StackDocumentError::invalid_name(project_path)
        })?;

        let mut yaml_vec = read_yml(&source_path).await;

        let yaml = Option::ok_or_else(yaml_vec.pop(), || {
            StackDocumentError::not_found(project_path)
        })?;

        let mapping = match yaml {
            YamlOwned::Mapping(mapping) => mapping,
            _ => return Err(StackDocumentError::not_valid(project_path)),
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

    pub async fn write(self) {
        let yaml = YamlOwned::Mapping(self.root);
        write_yml(&Yaml::from(&yaml), &self.output_path).await;
    }
}
