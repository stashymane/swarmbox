use std::path::{Path, PathBuf};
use util::walk_path;

pub struct Cache {
    pub files: Vec<PathBuf>,
}

impl Cache {
    pub async fn load(root: &Path, ignored: fn(&PathBuf) -> bool) -> Result<Self, String> {
        let files = walk_path(root)
            .await
            .map_err(|e| format!("Failed to read directory: {:?}", e))?
            .into_iter()
            .filter(|it| !ignored(it))
            .map(|absolute| {
                absolute
                    .strip_prefix(root)
                    .map(Path::to_owned)
                    .map_err(|_| format!("Failed to strip prefix: {:?}", absolute))
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Self { files })
    }

    pub fn list_files_in_dir(&self, dir: &Path) -> Vec<&PathBuf> {
        self.files.iter().filter(|it| it.starts_with(dir)).collect()
    }
}
