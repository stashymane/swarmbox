use shared::data::{Config, RelativePath};
use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use util::walk_path;

#[derive(Debug)]
pub struct Context {
    pub config: Config,
    pub sources: HashMap<String, RelativePath>,
    pub configs: HashMap<String, RelativePath>,
}

impl Context {
    pub async fn load(config: Config) -> Context {
        let sources = collect_paths(&config.paths.source, is_yml).await.unwrap();
        let configs = collect_paths(&config.paths.configs, |_| true)
            .await
            .unwrap();

        Context {
            config,
            sources,
            configs,
        }
    }
}

async fn collect_paths(
    dir: &PathBuf,
    filter: fn(&PathBuf) -> bool,
) -> Result<HashMap<String, RelativePath>, Error> {
    let mut sources = HashMap::<String, RelativePath>::new();

    walk_path(dir)
        .await?
        .into_iter()
        .filter(filter)
        .for_each(|path| {
            let project_path = RelativePath::from(&path, dir).unwrap();
            let name = project_path.name().unwrap();

            sources.insert(name, project_path);
        });

    Ok(sources)
}

fn is_yml(file: &PathBuf) -> bool {
    file.extension() == Some(std::ffi::OsStr::new("yml"))
        || file.extension() == Some(std::ffi::OsStr::new("yaml"))
}
