use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::deployment::{DeploymentType, ServerType};

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
