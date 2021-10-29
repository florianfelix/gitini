use std::path::PathBuf;

#[derive(Debug)]
pub struct CreateSettings {
    pub working_dir: PathBuf,
    pub repo_name: String,
    pub private: bool,
}

impl CreateSettings {
    pub fn new(working_dir: PathBuf, repo_name: String, private: bool) -> Self {
        Self {
            working_dir: working_dir,
            repo_name: repo_name,
            private: private
        }
    }
}