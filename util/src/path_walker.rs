use std::fs::{read_dir, DirEntry, ReadDir};
use std::io;
use std::path::{Path, PathBuf};

pub struct WalkPath {
    stack: Vec<ReadDir>,
}

impl Iterator for WalkPath {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let rd = self.stack.last_mut()?;

            match rd.next() {
                Some(Ok(entry)) => {
                    let path = entry.path();
                    if path.is_dir() {
                        match read_dir(&path) {
                            Ok(child) => self.stack.push(child),
                            Err(e) => return Some(Err(e)),
                        }
                        continue;
                    }
                    return Some(Ok(entry));
                }
                Some(Err(e)) => return Some(Err(e)),
                None => {
                    self.stack.pop();
                    continue;
                }
            }
        }
    }
}

impl WalkPath {
    pub fn map_to_paths(self) -> io::Result<Vec<PathBuf>> {
        self.map(|entry| entry.map(|e| e.path()))
            .collect::<io::Result<Vec<_>>>()
    }
}

pub fn walk_path(dir: &Path) -> io::Result<WalkPath> {
    if dir.is_dir() {
        Ok(WalkPath {
            stack: vec![read_dir(dir)?],
        })
    } else {
        Ok(WalkPath { stack: vec![] })
    }
}
