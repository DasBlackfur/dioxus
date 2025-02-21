use crate::Platform;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ApplicationConfig {
    #[serde(default = "default_platform")]
    pub(crate) default_platform: Platform,

    #[serde(default = "asset_dir_default")]
    pub(crate) asset_dir: PathBuf,

    #[serde(default)]
    pub(crate) sub_package: Option<String>,
}

pub(crate) fn default_platform() -> Platform {
    Platform::Web
}

pub(crate) fn asset_dir_default() -> PathBuf {
    PathBuf::from("assets")
}
