use std::path::PathBuf;

#[derive(Debug)]
pub struct RuntimeSettings {
    pub working_dir: PathBuf,
    pub repo_name: String,
    pub private: bool,
    pub complete: bool,
    pub open_website: bool,
}

impl RuntimeSettings {
    pub fn new(working_dir: PathBuf, repo_name: String, private: bool) -> Self {
        Self {
            working_dir: working_dir,
            repo_name: repo_name,
            private: private,
            complete: false,
            open_website: true,
        }
    }
}
