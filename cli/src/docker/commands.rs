use std::io;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("{0} (exit code {1})")]
    Failure(String, i32),
}

pub struct DockerCli {}

impl DockerCli {
    pub fn stack_config(
        paths: Vec<PathBuf>,
        skip_interpolation: bool,
    ) -> Result<String, DockerError> {
        let paths = paths
            .iter()
            .map(|path| path.canonicalize().map_err(DockerError::Io))
            .collect::<Result<Vec<PathBuf>, DockerError>>()?;

        let file_args = paths
            .iter()
            .flat_map(|path| ["-c".as_ref(), path.as_os_str()])
            .collect::<Vec<_>>();

        let mut command = Command::new("docker");
        command.arg("stack").arg("config").args(file_args);

        if skip_interpolation {
            command.arg("--skip-interpolation");
        }

        let output = command.output()?;

        if !output.status.success() {
            return Err(DockerError::Failure(
                parse_stdout(&output.stderr),
                output.status.code().unwrap_or(-1),
            ));
        }

        Ok(parse_stdout(&output.stdout))
    }
}

fn parse_stdout(output: &Vec<u8>) -> String {
    String::from_utf8_lossy(output).to_string()
}
