use crate::compose::modules::Module;
use crate::compose::state::State;
use crate::compose::util::{key_of, merge};
use log::debug;
use saphyr::{LoadableYamlNode, Mapping, Yaml, YamlEmitter};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;

pub fn generate_stacks(state: &State) {
    state.stacks.iter().for_each(|(_, stack)| {
        generate_stack(state, stack);
    })
}

pub fn generate_stack(state: &State, stack: &Module) {
    println!("Generating stack {:?}...", stack.name);

    let out_path = state.config.paths.resolve_out(&stack.path);
    debug!("Creating output path: {:?}", out_path.parent());
    create_dir_all(
        &out_path
            .parent()
            .expect("Failed to retrieve stack parent path"),
    )
    .expect("Could not create output directory");

    let source_path = state.config.paths.resolve_source(&stack.path);
    debug!("Reading source file: {:?}", source_path);

    let content = read_to_string(source_path).unwrap();
    let mut yaml_files = Yaml::load_from_str(&content).expect("Failed to parse stack");
    let yaml = &mut yaml_files.get_mut(0).expect("Stack is empty");

    match yaml {
        Yaml::Mapping(mapping) => {
            merge_includes(state, mapping);
        }
        _ => {
            panic!("Invalid stack file: {:?}", yaml_files[0]);
        }
    }

    let mut output = String::new();
    let mut emitter = YamlEmitter::new(&mut output);
    emitter.dump(yaml).expect("Failed to dump yaml");

    let mut output_file = File::create(out_path).expect("Failed to create output file");
    output_file
        .write_all(output.as_bytes())
        .expect("Failed to write output");
}

pub fn merge_includes(state: &State, yaml: &mut Mapping) {
    let key = &key_of("include");

    let Some(include) = yaml.get(key) else {
        debug!("Include section not found, continuing...");
        return;
    };
    let Some(contents) = include.as_sequence() else {
        debug!("Include is not a sequence, continuing...");
        return;
    };

    let include_names = contents
        .iter()
        .filter_map(|yaml| yaml.as_str())
        .map(|str| str.to_owned())
        .collect::<Vec<_>>();

    yaml.remove(key);

    for name in include_names.iter() {
        debug!("Including module {:?}", name);
        let module = state.modules.get(name).expect("Module not found");
        let content = read_to_string(state.config.paths.resolve_source(&module.path)).unwrap();
        let yaml_files = Yaml::load_from_str(&content).expect("Failed to parse stack");
        let include = yaml_files.get(0).expect("Stack is empty");
        if let Some(mapping) = include.as_mapping() {
            merge(yaml, mapping);
        }
    }
}
