use crate::commands::build::build_command;
use crate::commands::root::{Cli, Commands};
use clap::Parser;
use log::{debug, info};
use shared::data::Config;
use std::path::PathBuf;
use std::time::Instant;

mod commands;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let cli = Cli::parse();
    let project_path = &cli.project.unwrap_or(PathBuf::from("."));

    let config = Config::new(project_path).unwrap_or_else(|err| {
        eprintln!("Failed to load project configuration:");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
    debug!("Loaded config: {:?}", config);

    let before = Instant::now();
    let result = match cli.command {
        Commands::Build(args) => build_command(config, args).await,
    };

    if let Err(err) = result {
        eprintln!("Build failed: {:?}", err);
        std::process::exit(1);
    }

    let after = Instant::now();
    info!("Done in {:?}", after.duration_since(before));
}
