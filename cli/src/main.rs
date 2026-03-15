use crate::commands::build::build_command;
use crate::commands::main::{Cli, Commands};
use clap::Parser;
use log::{debug, info};
use shared::data::{Config, ConfigError};
use std::path::PathBuf;
use std::time::Instant;

mod commands;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let cli = Cli::parse();
    let config = Config::new(&cli.project.unwrap_or(PathBuf::from("."))).unwrap_or_else(|err| {
        match err {
            ConfigError::PathMissing(name, path) => {
                eprintln!("{} path missing: {:?}", name, path);
            }
        }
        std::process::exit(1);
    });
    debug!("Loaded config: {:?}", config);

    let before = Instant::now();
    match cli.command {
        Commands::Build(args) => build_command(config, args),
    }
    let after = Instant::now();
    info!("Done in {:?}", after.duration_since(before));
}
