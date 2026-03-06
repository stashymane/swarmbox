use crate::compose::context::Context;
use crate::compose::filters::includes::merge_includes;
use crate::compose::yaml_util::{read_yml, write_yml};
use saphyr::Yaml;

pub fn generate_stack(state: &Context, stack: &String) -> Result<(), String> {
    println!("Generating stack {:?}...", stack);

    let Some((name, project_path)) = state
        .sources
        .iter()
        .filter(|(name, _)| {
            name.strip_suffix(".yml")
                .or_else(|| name.strip_suffix(".yaml"))
                .map(|s| s == stack)
                .unwrap_or(false)
        })
        .next()
    else {
        return Err(format!("No stack found for {}", stack));
    };

    let source_path = project_path.get_full_path(&state.config.paths.source);
    let mut yaml_vec = read_yml(&source_path);
    let yaml = &mut yaml_vec[0];

    match yaml {
        Yaml::Mapping(mapping) => {
            merge_includes(state, mapping);
        }
        _ => {
            panic!("Invalid stack file: {:?}", name);
        }
    }

    let output_path = project_path.get_full_path(&state.config.paths.out);

    write_yml(yaml, &output_path);

    Ok(())
}
