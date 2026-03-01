use crate::compose::modules::Module;
use crate::data::config::Config;
use crate::util::fs::walk_path;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct State {
    pub config: Config,
    pub stacks: HashMap<String, Module>,
    pub modules: HashMap<String, Module>,
    pub configs: HashMap<String, Module>,
}

impl State {
    pub fn load(config: Config) -> State {
        let stacks = load_modules(&config.paths.get_stack_path(), &config.paths.source, is_yml);
        let modules = load_modules(
            &config.paths.get_module_path(),
            &config.paths.source,
            is_yml,
        );
        let configs = load_modules(
            &config.paths.get_config_path(),
            &config.paths.source,
            |_| true,
        );

        State {
            config,
            stacks,
            modules,
            configs,
        }
    }
}

fn load_modules(
    dir: &PathBuf,
    root: &PathBuf,
    filter: fn(&PathBuf) -> bool,
) -> HashMap<String, Module> {
    let mut modules = HashMap::<String, Module>::new();
    walk_path(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(filter)
        .for_each(|path| {
            let module = Module::from(&path, root);
            modules.insert(module.name.clone(), module);
        });
    modules
}

fn is_yml(file: &PathBuf) -> bool {
    file.extension() == Some(std::ffi::OsStr::new("yml"))
        || file.extension() == Some(std::ffi::OsStr::new("yaml"))
}
