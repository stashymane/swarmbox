use std::io;
use std::path::{Path, PathBuf, StripPrefixError};
use thiserror::Error;
use util::walk_path;

pub struct Cache {
    pub files: Vec<PathBuf>,
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Failed to read directory")]
    Io(#[from] io::Error),
    #[error("Failed to relativize path {0}")]
    Path(PathBuf, #[source] StripPrefixError),
}

impl Cache {
    pub async fn load(root: &Path, ignored: fn(&PathBuf) -> bool) -> Result<Self, CacheError> {
        let files = walk_path(root)
            .await
            .map_err(CacheError::Io)?
            .into_iter()
            .filter(|it| !ignored(it))
            .map(|absolute| {
                absolute
                    .strip_prefix(root)
                    .map(Path::to_owned)
                    .map_err(|e| CacheError::Path(absolute, e))
            })
            .collect::<Result<Vec<_>, CacheError>>()?;

        Ok(Self { files })
    }

    pub fn list_files_in_dir(&self, dir: &Path) -> Vec<&PathBuf> {
        self.files.iter().filter(|it| it.starts_with(dir)).collect()
    }
}
