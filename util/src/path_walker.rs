use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::{read_dir, ReadDir};

pub async fn walk_path(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    if !dir.is_dir() {
        return Ok(paths);
    }

    let mut stack: Vec<ReadDir> = vec![read_dir(dir).await?];

    while let Some(mut rd) = stack.pop() {
        while let Ok(Some(entry)) = rd.next_entry().await {
            let path = entry.path();
            if path.is_dir() {
                stack.push(rd);
                rd = read_dir(&path).await?;
            } else {
                paths.push(path);
            }
        }
    }

    Ok(paths)
}
