use crate::compose::state::State;
use crate::compose::util::{key_of, merge};
use log::debug;
use saphyr::{LoadableYamlNode, Mapping, Yaml};
use std::fs::read_to_string;

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
        debug!("Merging module {:?}", name);
        let module = state.modules.get(name).expect("Module not found");
        let content = read_to_string(state.config.paths.resolve_source(&module.path)).unwrap();
        let yaml_files = Yaml::load_from_str(&content).expect("Failed to parse stack");
        let include = yaml_files.get(0).expect("Stack is empty");
        if let Some(mapping) = include.as_mapping() {
            merge(yaml, mapping);
        }
    }
}
