use std::{fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use super::deployment::{DeploymentType, ServerType};

pub const CONFIG_FILE: &str = "jd.json";

/// Persisted configuration written to jd.json
#[derive(Serialize, Deserialize, Debug)]
pub struct JdConfig {
    pub name: String,
    pub version: String,
    pub project_dir: PathBuf,
    pub deployment_type: DeploymentType,
    pub file_path: PathBuf,
    pub server: ServerType,
}

impl JdConfig {
    pub fn load() -> Result<Self> {
        let path = PathBuf::from(CONFIG_FILE);
        if !path.exists() {
            bail!(
                "No configuration file found for this project. Run `jd setup` first."
            );
        }
        let contents = fs::read_to_string(&path)
            .context("Failed to read jd.json")?;
        serde_json::from_str(&contents).context("Failed to parse jd.json")
    }
}
