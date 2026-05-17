use crate::docker::commands::DockerCli;
use anyhow::{anyhow, Context};
use clap::Args;
use log::{debug, info};
use processing::data::context::ProcessingContext;
use shared::data::{Config, RelativePath};
use std::fs::write;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, remove_dir_all};

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Names of the stacks to build
    stacks: Vec<String>,
    /// Where the built files should be placed
    #[arg(short, long, default_value = "./out")]
    output: PathBuf,
    /// Delete the current "out" directory before building
    #[arg(long, default_value = "false")]
    overwrite: bool,
    /// Watch for project changes and rebuild automatically
    #[arg(long, default_value = "false")]
    watch: bool,
    /// Validate the stack after building it with Docker CLI
    #[arg(short, long, default_value = "false")]
    validate: bool,
}

pub async fn build_command(config: Config, args: BuildArgs) -> anyhow::Result<()> {
    initialize_out(&config.paths.out, args.overwrite).await?;
    let context = ProcessingContext::load(config).await?;

    let stacks = if args.stacks.is_empty() {
        vec!["stack".to_string()]
    } else {
        args.stacks
    };

    let stacks = stacks
        .into_iter()
        .map(resolve_stack)
        .map(|it| verify_or_err(it, &context.config))
        .collect::<anyhow::Result<Vec<_>>>()?;

    for stack in stacks.into_iter() {
        let path = Option::ok_or_else(RelativePath::new(stack.to_owned()), || {
            anyhow!("Path {:?} must be relative", stack)
        })?;

        info!("Processing {:?}...", stack);
        let stack_path = context
            .process(&path)
            .await
            .with_context(|| format!("Failed to process stack {:?}", stack))?;
        println!("Processed {:?}", stack);

        if args.validate {
            debug!("Validating {:?}...", stack);
            match DockerCli::stack_config(vec![stack_path], false) {
                Ok(_) => {
                    println!("{:?} validated successfully", stack);
                }
                Err(e) => {
                    eprintln!("Failed to validate {:?}:", stack);
                    eprintln!("{}", e);
                }
            }
        }
    }

    Ok(())
}

async fn initialize_out(path: &PathBuf, overwrite: bool) -> anyhow::Result<()> {
    if path.exists() {
        if overwrite {
            remove_dir_all(path)
                .await
                .context("Failed to delete out directory")?;
        } else {
            return Err(anyhow!(
                "Output path already exists: {}",
                path.to_str().unwrap()
            ));
        }
    }

    create_dir_all(path)
        .await
        .context("Failed to create out directory")?;

    write(path.join(".gitignore"), "*").context("Failed to create .gitignore file")?;

    Ok(())
}

fn resolve_stack(name: String) -> PathBuf {
    let mut path = PathBuf::from(name);
    if path.extension().is_none() {
        path = path.with_extension("yml")
    }

    path
}

fn verify_or_err(path: PathBuf, config: &Config) -> anyhow::Result<PathBuf> {
    let file_path = config.paths.source.join(&path);
    if !file_path.exists() {
        return Err(anyhow!("{:?} does not exist at {:?}", &path, &file_path));
    }
    if !file_path.is_file() {
        return Err(anyhow!("{:?} must be a file, not a directory", &path));
    }

    Ok(path)
}
