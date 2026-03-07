use log::debug;
use saphyr::{LoadableYamlNode, Yaml, YamlEmitter, YamlOwned};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn read_yml(path: &PathBuf) -> Vec<YamlOwned> {
    debug!("Reading source file: {:?}", path);

    let mut file = File::open(path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read content");

    let yaml_files = YamlOwned::load_from_str(&content).expect("Failed to parse stack");
    yaml_files
}

pub fn write_yml(yaml: &YamlOwned, path: &PathBuf) {
    debug!("Creating output path: {:?}", path.parent());
    create_dir_all(path.parent().expect("Failed to retrieve stack parent path"))
        .expect("Could not create output directory");

    let mut output = String::new();
    let mut emitter = YamlEmitter::new(&mut output);
    emitter
        .dump(&Yaml::from(yaml))
        .expect("Failed to dump yaml");

    let mut output_file = File::create(path).expect("Failed to create output file");
    output_file
        .write_all(output.as_bytes())
        .expect("Failed to write output");
}
