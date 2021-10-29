use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GitifyConfig {
    pub api_key: String,
}

/// `GitifyConfig` implements `Default`
impl ::std::default::Default for GitifyConfig {
    fn default() -> Self {
        Self {
            api_key: "".into(),
        }
    }
}
