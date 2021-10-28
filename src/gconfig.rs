use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GitifyConfig {
    version: String,
    pub api_key: String,
}

/// `GitifyConfig` implements `Default`
impl ::std::default::Default for GitifyConfig {
    fn default() -> Self {
        Self {
            version: "0.1".into(),
            api_key: "".into(),
        }
    }
}
