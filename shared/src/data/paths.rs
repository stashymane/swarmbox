use std::ffi::OsStr;
use std::ops::Deref;
use std::path::{Path, PathBuf, StripPrefixError};

#[derive(Debug)]
pub struct RelativePath {
    inner: PathBuf,
}

impl Deref for RelativePath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl RelativePath {
    pub fn new(inner: PathBuf) -> Option<RelativePath> {
        if !inner.is_relative() {
            return None;
        }
        Some(RelativePath { inner })
    }

    pub fn from(path: &Path, root: &Path) -> Result<RelativePath, StripPrefixError> {
        let path = path.strip_prefix(root)?.to_owned();
        Ok(RelativePath { inner: path })
    }

    pub fn get_absolute_path(&self, root: &Path) -> PathBuf {
        root.join(&self.inner)
    }

    pub fn name(&self) -> Option<String> {
        let segments = self
            .inner
            .iter()
            .map(OsStr::to_str)
            .collect::<Option<Vec<_>>>()?;

        Some(segments.join("/"))
    }

    pub fn as_path(&self) -> &Path {
        self.inner.as_path()
    }
}
