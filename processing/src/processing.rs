use crate::data::context::Context;
use crate::data::stacks::StackDocument;
use crate::processors::configs::process_configs;
use crate::processors::includes::merge_includes;
use crate::yaml::write_yml;
use saphyr::YamlOwned;
use shared::data::RelativePath;

use std::future::Future;
use std::pin::Pin;

type Processor = for<'a> fn(
    &'a mut StackDocument,
    &'a Context,
) -> Pin<Box<dyn Future<Output = Result<(), String>> + 'a>>;

pub async fn generate_stack(context: &Context, stack: &String) -> Result<String, String> {
    println!("Generating stack {:?}...", stack);
    let processors: [Processor; 2] = [
        |doc, context| Box::pin(merge_includes(doc, context)),
        |doc, context| Box::pin(process_configs(doc, context)),
    ];

    let (name, _) = find_stack_by_name(context, stack)
        .ok_or_else(|| format!("Could not find {}.yml", stack))?;

    let mut doc = StackDocument::load(name, context)
        .await
        .or_else(|e| Err(format!("Failed to load stack \"{}\": {:?}", stack, e)))?;

    for processor in processors {
        processor(&mut doc, context).await?;
    }

    write_yml(&YamlOwned::Mapping(doc.root), &doc.output_path).await;

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
