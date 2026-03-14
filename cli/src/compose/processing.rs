use crate::compose::context::Context;
use crate::compose::data::paths::RelativePath;
use crate::compose::processors::configs::process_configs;
use crate::compose::processors::includes::merge_includes;
use crate::compose::stacks::StackDocument;
use crate::compose::yaml::write_yml;
use saphyr::YamlOwned;

type Processor = fn(&mut StackDocument, &Context) -> Result<(), String>;

pub fn generate_stack(context: &Context, stack: &String) -> Result<String, String> {
    println!("Generating stack {:?}...", stack);
    let processors: [Processor; 2] = [merge_includes, process_configs];

    let (name, project_path) = find_stack_by_name(context, stack)
        .ok_or_else(|| format!("Could not find {}.yml", stack))?;

    let mut doc = StackDocument::load(name, context)
        .or_else(|e| Err(format!("Failed to load stack \"{}\": {:?}", stack, e)))?;

    for processor in processors {
        processor(&mut doc, context)?;
    }

    write_yml(&YamlOwned::Mapping(doc.root), &doc.output_path);

    Ok(name.to_string())
}

fn find_stack_by_name<'a>(
    state: &'a Context,
    stack_name: &String,
) -> Option<(&'a String, &'a RelativePath)> {
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
