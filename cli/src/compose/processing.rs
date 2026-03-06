use crate::compose::context::Context;
use crate::compose::filters::includes::merge_includes;
use crate::compose::yaml_util::{read_yml, write_yml};
use saphyr::Yaml;

pub fn generate_stacks(state: &Context) {
    for (name, project_path) in state.sources.iter() {
        println!("Generating stack {:?}...", name);

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
    }
}
