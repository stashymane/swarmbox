use log::debug;
use saphyr::{LoadableYamlNode, Mapping, Yaml, YamlEmitter};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn merge<'a>(target: &mut Mapping<'a>, source: &Mapping<'a>) {
    for (key, src_value) in source {
        match target.get_mut(key) {
            Some(dst_value) => match (dst_value, src_value) {
                (Yaml::Mapping(dst_map), Yaml::Mapping(src_map)) => {
                    merge(dst_map, src_map);
                }
                (dst_value, src_value) => {
                    *dst_value = src_value.clone();
                }
            },
            None => {
                target.insert(key.clone(), src_value.clone());
            }
        }
    }
}

#[inline(always)]
pub fn key_of(key: &'_ str) -> Yaml<'_> {
    Yaml::value_from_str(key)
}

trait YamlExt {}
impl YamlExt for Yaml<'_> {}

pub fn read_yml<'a>(path: &PathBuf) -> Vec<Yaml<'a>> {
    debug!("Reading source file: {:?}", path);

    let mut file = File::open(path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read content");

    let yaml_files = Yaml::load_from_str(&content).expect("Failed to parse stack");
    yaml_files
}

pub fn write_yml(yaml: &Yaml, path: &PathBuf) {
    debug!("Creating output path: {:?}", path.parent());
    create_dir_all(path.parent().expect("Failed to retrieve stack parent path"))
        .expect("Could not create output directory");

    let mut output = String::new();
    let mut emitter = YamlEmitter::new(&mut output);
    emitter.dump(yaml).expect("Failed to dump yaml");

    let mut output_file = File::create(path).expect("Failed to create output file");
    output_file
        .write_all(output.as_bytes())
        .expect("Failed to write output");
}
