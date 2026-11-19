use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            name: "horatio-scaffold".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
