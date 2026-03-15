use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    pub paths: Paths,
}

#[derive(Debug)]
pub struct Paths {
    pub root: PathBuf,
    pub source: PathBuf,
    pub configs: PathBuf,
    pub out: PathBuf,
}

#[derive(Debug)]
pub enum ConfigError {
    PathMissing(String),
    InvalidPath(String),
}

impl ConfigError {
    fn path_missing(name: &str, path: &Path) -> ConfigError {
        ConfigError::PathMissing(format!("{} path missing: {:?}", name, path))
    }
    fn invalid_path(path: &Path, err: &io::Error) -> ConfigError {
        ConfigError::InvalidPath(format!(
            "Project root \"{:?}\" is not valid: {:?}",
            path, err
        ))
    }
}

impl Config {
    pub fn new(project_path: &PathBuf) -> Result<Config, ConfigError> {
        if !project_path.exists() {
            return Err(ConfigError::path_missing("Project", project_path));
        }

        Ok(Config {
            paths: Paths::from_root(project_path.clone())?,
        })
    }
}

impl Paths {
    pub fn from_root(root: PathBuf) -> Result<Paths, ConfigError> {
        let root = Result::map_err(root.canonicalize(), |err| {
            ConfigError::invalid_path(&root, &err)
        })?;

        let source_path = root.join("src");
        if !source_path.exists() {
            return Err(ConfigError::path_missing("Source", &source_path));
        }
        let config_path = root.join("configs");
        let out_path = root.join("out");

        Ok(Paths {
            root: root.clone(),
            source: source_path,
            configs: config_path,
            out: out_path,
        })
    }
}
