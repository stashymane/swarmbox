use crate::data::config::Config;
use crate::util::fs::walk_path;
use std::collections::HashMap;
use std::io::Error;
use std::path::{Path, PathBuf, StripPrefixError};

#[derive(Debug)]
pub struct ProjectPath {
    inner: PathBuf,
}

impl ProjectPath {
    pub fn from(path: &Path, root: &Path) -> Result<ProjectPath, StripPrefixError> {
        let path = path.strip_prefix(root)?.to_owned();
        Ok(ProjectPath { inner: path })
    }

    pub fn get_full_path(&self, root: &Path) -> PathBuf {
        root.join(&self.inner)
    }

    pub fn name(&self) -> Option<String> {
        let segments = self
            .inner
            .iter()
            .map(|os_str| os_str.to_str())
            .collect::<Option<Vec<_>>>()?;

        Some(segments.join("/"))
    }
}

#[derive(Debug)]
pub struct Context {
    pub config: Config,
    pub sources: HashMap<String, ProjectPath>,
    pub configs: HashMap<String, ProjectPath>,
}

impl Context {
    pub fn load(config: Config) -> Context {
        let sources = collect_paths(&config.paths.source, is_yml).unwrap();
        let configs = collect_paths(&config.paths.configs, |_| true).unwrap();

        Context {
            config,
            sources,
            configs,
        }
    }
}

fn collect_paths(
    dir: &PathBuf,
    filter: fn(&PathBuf) -> bool,
) -> Result<HashMap<String, ProjectPath>, Error> {
    let mut sources = HashMap::<String, ProjectPath>::new();

    let paths = walk_path(dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()?;

    paths.into_iter().filter(filter).for_each(|path| {
        let project_path = ProjectPath::from(&path, dir).unwrap();
        let name = project_path.name().unwrap();

        sources.insert(name, project_path);
    });

    Ok(sources)
}

fn is_yml(file: &PathBuf) -> bool {
    file.extension() == Some(std::ffi::OsStr::new("yml"))
        || file.extension() == Some(std::ffi::OsStr::new("yaml"))
}
