use crate::compose::context::{Context, ProjectPath};
use crate::compose::yaml_util::value_owned;
use saphyr::{MappingOwned, YamlOwned};
use sha2::{Digest, Sha256};
use std::fs::read;

pub fn process_configs(context: &Context, yaml: &mut MappingOwned) {
    let mut mapping = MappingOwned::new();

    for (_, project_path) in context.configs.iter() {
        let key = safe_config_name(project_path).unwrap();
        let config_path = project_path.get_full_path(&context.config.paths.configs);
        let hashed_name = hashed_config_name(&key, config_path.as_path()).unwrap();

        let mut content = MappingOwned::new();
        content.insert(value_owned("name".to_string()), value_owned(hashed_name));
        content.insert(
            value_owned("file".to_string()),
            value_owned(config_path.to_string_lossy().to_string()),
        );

        mapping.insert(value_owned(key), YamlOwned::Mapping(content));
    }

    yaml.insert(
        value_owned("configs".to_string()),
        YamlOwned::Mapping(mapping),
    );
}

pub fn safe_config_name(path: &ProjectPath) -> Option<String> {
    Some(path.name()?.replace("/", "_"))
}

fn hashed_config_name(base: &str, full_path: &std::path::Path) -> Option<String> {
    let bytes = read(full_path).ok()?;

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();

    let short_hash = format!("{:x}", digest);
    Some(format!("{}-{}", base, &short_hash[..12]))
}
