use crate::compose::filters::includes::merge_includes;
use crate::compose::modules::Module;
use crate::compose::state::State;
use saphyr::Yaml;

pub fn generate_stacks(state: &State) {
    state.stacks.iter().for_each(|(_, stack)| {
        generate_stack(state, stack);
    })
}

pub fn generate_stack(state: &State, stack: &Module) {
    println!("Generating stack {:?}...", stack.name);

    let mut yaml_vec = stack.open(state);
    let yaml = &mut yaml_vec[0];

    match yaml {
        Yaml::Mapping(mapping) => {
            merge_includes(state, mapping);
        }
        _ => {
            panic!("Invalid stack file: {:?}", stack.name);
        }
    }

    stack.write(state, yaml);
}
