use std::path::PathBuf;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub path: PathBuf,
}

impl Module {
    pub fn from(path: &PathBuf, project_root: &PathBuf) -> Module {
        let relative = path.strip_prefix(project_root).unwrap().to_path_buf();
        let name = relative.iter().fold(String::new(), |acc, segment| {
            if acc.is_empty() {
                format!("{}", segment.to_str().unwrap())
            } else {
                format!("{}/{}", acc, segment.to_str().unwrap())
            }
        });
        Module {
            name,
            path: relative,
        }
    }
}
