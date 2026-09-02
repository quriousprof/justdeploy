use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Deployment Representation
///
/// This struct contains all the information about a deployment
#[derive(Serialize, Deserialize)]
pub struct Deployment {
    name: String,
    file_location: String,
    config_location: String,
    deployment_type: DeploymentType,
    server: ServerType,
    deployed_at: DateTime<Utc>,
    status: DeploymentStatus,
}

impl Deployment {
    pub fn new(
        name: String,
        file_location: String,
        config_location: String,
        deployment_type: DeploymentType,
        server: ServerType,
    ) -> Self {
        Self {
            name,
            file_location,
            config_location,
            deployment_type,
            server,
            deployed_at: Utc::now(),
            status: DeploymentStatus::Idle,
        }
    }
}

/// Type of deployment
///
/// Can be Dockerfile, DockerCompose (more yet to come)
#[derive(Serialize, Deserialize)]
pub enum DeploymentType {
    Dockerfile,
    DockerCompose,
}

/// Server Type
///
/// Can be Local / Remote (ipv6)
#[derive(Serialize, Deserialize)]
pub enum ServerType {
    Local,
    Remote(String),
}

/// Status of deployment
///
/// This is handled internally to keep a track of the deployment
#[derive(Serialize, Deserialize)]
pub enum DeploymentStatus {
    Idle,
    Starting,
    Running,
    Stopped,
}
