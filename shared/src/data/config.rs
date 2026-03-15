use std::path::PathBuf;

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
    PathMissing(String, PathBuf),
}

impl Config {
    pub fn new(project: &PathBuf) -> Result<Config, ConfigError> {
        let project_path = project.clone();
        if !project_path.exists() {
            return Err(ConfigError::PathMissing(
                "Project".to_string(),
                project_path,
            ));
        }

        Ok(Config {
            paths: Paths::from_root(project.clone())?,
        })
    }
}

impl Paths {
    pub fn from_root(root: PathBuf) -> Result<Paths, ConfigError> {
        let source_path = root.join("src");
        if !source_path.exists() {
            return Err(ConfigError::PathMissing("Source".to_string(), source_path));
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
