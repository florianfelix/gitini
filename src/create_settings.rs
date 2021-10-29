use std::path::PathBuf;

pub struct CreateSettings {
    working_dir: PathBuf,
    private: bool,
}

impl CreateSettings {
    pub fn new(working_dir: PathBuf, private: bool) -> Self {
        Self {
            working_dir: working_dir,
            private: private
        }
    }
}