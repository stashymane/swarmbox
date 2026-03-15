use crate::data::context::Context;
use crate::data::stacks::StackDocument;
use crate::yaml::{MappingExt, YamlOwnedExt};
use log::debug;
use saphyr::{MappingOwned, YamlOwned};

pub fn merge_includes(doc: &mut StackDocument, context: &Context) -> Result<(), String> {
    merge_includes_recursive(context, &mut doc.root, &mut Vec::new())?;
    Ok(())
}

fn merge_includes_recursive(
    context: &Context,
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

    let include_names = contents
        .iter()
        .filter_map(|yaml| yaml.as_str())
        .map(|str| str.to_owned())
        .collect::<Vec<_>>();

    yaml.remove(&key);

    for name in include_names.iter() {
        if stack.contains(name) {
            return Err(format!("Circular include detected: {:?}", name));
        }

        debug!("Merging module {:?}", name);
        let mut doc = StackDocument::load(name, context)
            .map_err(|e| format!("Failed to merge include: {:?}", e))?;

        stack.push(name.to_string());
        merge_includes_recursive(context, &mut doc.root, stack)?;

        yaml.merge_from(&doc.root);
    }

    Ok(())
}
