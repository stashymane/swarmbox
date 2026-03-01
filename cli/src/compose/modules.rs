use crate::compose::state::State;
use log::debug;
use saphyr::{LoadableYamlNode, Yaml, YamlEmitter};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub path: PathBuf,
}

impl Module {
    pub fn from(path: &PathBuf, project_root: &PathBuf) -> Module {
        let relative = path.strip_prefix(project_root).unwrap().to_path_buf();
        let name = relative.iter().fold(String::new(), |acc, segment| {
            if acc.is_empty() {
                format!("{}", segment.to_str().unwrap())
            } else {
                format!("{}/{}", acc, segment.to_str().unwrap())
            }
        });
        Module {
            name,
            path: relative,
        }
    }

    pub fn open(&self, state: &State) -> Vec<Yaml> {
        let source_path = state.config.paths.resolve_source(&self.path);
        debug!("Reading source file: {:?}", source_path);

        let content = read_to_string(source_path).unwrap();
        let yaml_files = Yaml::load_from_str(&content).expect("Failed to parse stack");
        yaml_files
    }

    pub fn write(&self, state: &State, yaml: &Yaml) {
        let out_path = state.config.paths.resolve_out(&self.path);
        debug!("Creating output path: {:?}", out_path.parent());
        create_dir_all(
            &out_path
                .parent()
                .expect("Failed to retrieve stack parent path"),
        )
        .expect("Could not create output directory");

        let mut output = String::new();
        let mut emitter = YamlEmitter::new(&mut output);
        emitter.dump(yaml).expect("Failed to dump yaml");

        let mut output_file = File::create(out_path).expect("Failed to create output file");
        output_file
            .write_all(output.as_bytes())
            .expect("Failed to write output");
    }
}
