use std::path::{Path, PathBuf, StripPrefixError};

#[derive(Debug)]
pub struct RelativePath {
    inner: PathBuf,
}

impl RelativePath {
    pub fn from(path: &Path, root: &Path) -> Result<RelativePath, StripPrefixError> {
        let path = path.strip_prefix(root)?.to_owned();
        Ok(RelativePath { inner: path })
    }

    pub fn get_full_path(&self, root: &Path) -> PathBuf {
        root.join(&self.inner)
    }

    pub fn name(&self) -> Option<String> {
        let segments = self
            .inner
            .iter()
            .map(|os_str| os_str.to_str())
            .collect::<Option<Vec<_>>>()?;

        Some(segments.join("/"))
    }
}
