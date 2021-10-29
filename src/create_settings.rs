use std::path::PathBuf;

#[derive(Debug)]
pub struct CreateSettings {
    pub working_dir: PathBuf,
    pub private: bool,
}

impl CreateSettings {
    pub fn new(working_dir: PathBuf, private: bool) -> Self {
        Self {
            working_dir: working_dir,
            private: private
        }
    }
}