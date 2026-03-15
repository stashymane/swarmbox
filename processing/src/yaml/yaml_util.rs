use log::debug;
use saphyr::{LoadableYamlNode, Yaml, YamlEmitter, YamlOwned};
use std::path::PathBuf;
use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn read_yml(path: &PathBuf) -> Vec<YamlOwned> {
    debug!("Reading source file: {:?}", path);

    let mut file = File::open(path).await.expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .await
        .expect("Failed to read content");

    let yaml_files = YamlOwned::load_from_str(&content).expect("Failed to parse stack");
    yaml_files
}

pub async fn write_yml(yaml: &Yaml<'_>, path: &PathBuf) {
    debug!("Creating output path: {:?}", path.parent());
    create_dir_all(path.parent().expect("Failed to retrieve stack parent path"))
        .await
        .expect("Could not create output directory");

    let mut output = String::new();
    let mut emitter = YamlEmitter::new(&mut output);
    emitter.dump(yaml).expect("Failed to dump yaml");

    let mut output_file = File::create(path)
        .await
        .expect("Failed to create output file");
    output_file
        .write_all(output.as_bytes())
        .await
        .expect("Failed to write output");
}
