use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub paths: Paths,
}

#[derive(Debug)]
pub struct Paths {
    pub root: PathBuf,
    pub source: PathBuf,
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
        let out_path = root.join("out");

        Ok(Paths {
            root: root.clone(),
            source: source_path,
            out: out_path,
        })
    }

    pub fn resolve_source(&self, path: &PathBuf) -> PathBuf {
        self.source.join(path)
    }

    pub fn resolve_out(&self, path: &PathBuf) -> PathBuf {
        self.out.join(path)
    }

    pub fn get_config_path(&self) -> PathBuf {
        self.source.join("configs")
    }

    pub fn get_module_path(&self) -> PathBuf {
        self.source.join("modules")
    }

    pub fn get_stack_path(&self) -> PathBuf {
        self.source.join("stacks")
    }
}
