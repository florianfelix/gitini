use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSettings {
    pub api_key: String,
}

/// `ConfigSettings` implements `Default`
impl ::std::default::Default for ConfigSettings {
    fn default() -> Self {
        Self { api_key: "".into() }
    }
}
