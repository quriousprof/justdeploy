use std::{fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use super::deployment::{DeploymentType, ServerType};

pub const CONFIG_FILE: &str = "jd.json";

/// Extra arguments applied when running the deployment
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeploymentArgs {
    /// Port mappings e.g. ["8000:8000", "443:443"]
    #[serde(default)]
    pub ports: Vec<String>,
    /// Path to an env file passed via --env-file
    #[serde(default)]
    pub env_file: Option<String>,
    /// Restart policy e.g. "unless-stopped", "always", "on-failure"
    #[serde(default)]
    pub restart: Option<String>,
    /// Volume mounts e.g. ["./data:/app/data"]
    #[serde(default)]
    pub volumes: Vec<String>,
    /// Extra environment variables e.g. ["KEY=VALUE"]
    #[serde(default)]
    pub env: Vec<String>,
}

/// Persisted configuration written to jd.json
#[derive(Serialize, Deserialize, Debug)]
pub struct JdConfig {
    pub name: String,
    pub version: String,
    pub project_dir: PathBuf,
    pub deployment_type: DeploymentType,
    pub file_path: PathBuf,
    pub server: ServerType,
    /// Arguments applied at deploy time (edit manually before running jd deploy)
    #[serde(default)]
    pub deployment_args: DeploymentArgs,
}

impl JdConfig {
    pub fn load() -> Result<Self> {
        let path = PathBuf::from(CONFIG_FILE);
        if !path.exists() {
            bail!("No configuration file found for this project. Run `jd setup` first.");
        }
        Self::load_from(&path)
    }

    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read '{}'", path.display()))?;
        serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse '{}'", path.display()))
    }
}
