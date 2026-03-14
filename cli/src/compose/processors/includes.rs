use crate::compose::context::Context;
use crate::compose::stacks::StackDocument;
use crate::compose::yaml::{MappingExt, YamlOwnedExt};
use log::debug;
use saphyr::{LoadableYamlNode, MappingOwned, YamlOwned};
use std::fs::read_to_string;

pub fn merge_includes(doc: &mut StackDocument, context: &Context) -> Result<(), String> {
    merge_includes_recursive(context, &mut doc.root)?;
    Ok(())
}

fn merge_includes_recursive(context: &Context, yaml: &mut MappingOwned) -> Result<(), String> {
    let key = YamlOwned::value_of("include");

    let Some(include) = yaml.get(&key) else {
        debug!("Include section not found, continuing...");
        return Ok(());
    };
    let Some(contents) = include.as_sequence() else {
        debug!("Include is not a sequence, continuing...");
        return Ok(());
    };

    let include_names = contents
        .iter()
        .filter_map(|yaml| yaml.as_str())
        .map(|str| str.to_owned())
        .collect::<Vec<_>>();

    yaml.remove(&key);

    for name in include_names.iter() {
        debug!("Merging module {:?}", name);
        let project_path = context.sources.get(name).expect("Module not found");
        let source_path = project_path.get_full_path(&context.config.paths.source);

        let content = read_to_string(&source_path).or_else(|err| {
            Err(format!(
                "Failed to read included file \"{:?}\": {}",
                source_path, err
            )
            .to_string())
        })?;
        let mut yaml_files = YamlOwned::load_from_str(&content).expect("Failed to parse stack");
        let include = yaml_files.get_mut(0).expect("Stack is empty");
        if let Some(mapping) = include.as_mapping_mut() {
            merge_includes_recursive(context, mapping)?;
            yaml.merge_from(mapping);
        }
    }

    Ok(())
}
