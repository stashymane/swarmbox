use clap::Args;
use processing::data::context::ProcessingContext;
use shared::data::{Config, RelativePath};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Names of the stacks to build
    stacks: Vec<String>,
    /// Where the built files should be placed
    #[arg(short, long, default_value = "./out")]
    output: PathBuf,
    /// Watch for project changes and rebuild automatically
    #[arg(long, default_value = "false")]
    watch: bool,
}

pub async fn build_command(config: Config, args: BuildArgs) -> Result<(), String> {
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
        .collect::<Result<Vec<_>, String>>()?;

    for stack in stacks.into_iter() {
        let path = Option::ok_or_else(RelativePath::new(stack.to_owned()), || {
            format!("Path {:?} must be relative", stack)
        })?;

        println!("Processing {:?}", stack);
        let result = context.process(&path).await?;
    }

    Ok(())
}

fn resolve_stack(name: String) -> PathBuf {
    let mut path = PathBuf::from(name);
    if path.extension().is_none() {
        path = path.with_extension("yml")
    }

    path
}

fn verify_or_err(path: PathBuf, config: &Config) -> Result<PathBuf, String> {
    let file_path = config.paths.source.join(&path);
    if !file_path.exists() {
        return Err(format!("Stack \"{:?}\" does not exist", &path));
    }
    if !file_path.is_file() {
        return Err(format!(
            "Stack \"{:?}\" must be a file, not a directory",
            &path
        ));
    }

    Ok(path)
}
