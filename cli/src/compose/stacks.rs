use crate::compose::context::Context;
use crate::compose::yaml::read_yml;
use saphyr::{MappingOwned, YamlOwned};
use std::path::PathBuf;
pub struct StackDocument {
    pub stack_name: String,
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub root: MappingOwned,
}

pub enum StackDocumentError {
    NotFound,
    Invalid,
}

impl StackDocument {
    pub fn load(name: &str, context: &Context) -> Result<StackDocument, StackDocumentError> {
        let project_path = context
            .sources
            .get(name)
            .ok_or_else(|| StackDocumentError::NotFound)?;
        let source_path = project_path.get_full_path(&context.config.paths.source);
        let mut yaml_vec = read_yml(&source_path);
        let yaml = yaml_vec.pop().ok_or_else(|| StackDocumentError::Invalid)?;

        let mapping = match yaml {
            YamlOwned::Mapping(mapping) => mapping,
            _ => return Err(StackDocumentError::Invalid),
        };

        let output_path = project_path.get_full_path(&context.config.paths.out);

        let doc = StackDocument {
            stack_name: name.to_owned(),
            source_path,
            output_path: output_path.clone(),
            root: mapping,
        };

        Ok(doc)
    }
}
