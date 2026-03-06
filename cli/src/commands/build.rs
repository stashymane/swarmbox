use crate::compose::context::Context;
use crate::compose::processing::generate_stacks;
use crate::data::config::Config;
use clap::Args;
use log::debug;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Where the built files should be placed
    #[arg(short, long, default_value = "./out")]
    output: PathBuf,
    /// Watch for project changes and rebuild automatically
    #[arg(long, default_value = "false")]
    watch: bool,
}

pub fn build_command(config: Config, args: BuildArgs) {
    let state = Context::load(config);
    debug!("Loaded state: {:?}", state);

    generate_stacks(&state);
}
