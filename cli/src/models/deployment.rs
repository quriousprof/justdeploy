use std::{io::{BufRead, BufReader}, path::PathBuf, process::{Command, Stdio}};

use anyhow::{Result, bail};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::parse_file;

/// Deployment Representation
///
/// This struct contains all the information about a deployment
#[derive(Serialize, Deserialize, Debug)]
pub struct Deployment {
    name: String,
    file_path: PathBuf,
    config_location: String,
    deployment_type: DeploymentType,
    server: ServerType,
    deployed_at: DateTime<Utc>,
    status: DeploymentStatus,
}

impl Deployment {
    pub fn new(
        name: String,
        file_path: PathBuf,
        config_location: String,
        server: ServerType,
    ) -> Self {
        
        // checking file contents
        let deployment_type: DeploymentType = parse_file(&file_path).expect("failed");

        Self {
            name,
            file_path: file_path,
            config_location,
            deployment_type,
            server,
            deployed_at: Utc::now(),
            status: DeploymentStatus::Idle,
        }
    }

    pub fn build(self) {
        println!("Building {}...", self.name);
        
        // building according to the deployment type
        match self.deployment_type {
            DeploymentType::Dockerfile => {
                // bulding the docker file 
                let mut build_cmd = Command::new("docker");
                build_cmd.arg("build");
                if !self.name.is_empty() {
                    build_cmd.args(["-t", self.name.as_str()]);
                }
                build_cmd.args(["-f", self.file_path.to_str().unwrap()]);
                build_cmd.arg(self.file_path.parent().unwrap().to_str().unwrap());

                // TODO: Add error handling for output
                let mut final_cmd = build_cmd.stdout(Stdio::piped()).spawn().unwrap();

                {
                    let stdout = final_cmd.stdout.as_mut().unwrap();
                    let stdout_lines = BufReader::new(stdout).lines();

                    for line in stdout_lines {
                        println!("[STDOUT] {:?}", line);
                    }
                }

                // TODO: handle exit codes
                final_cmd.wait().unwrap();

                println!("CMD: {:?}", &build_cmd);
                
                println!("Dockerfile built successfully!");
            } 
            DeploymentType::DockerCompose => todo!(),
        }
    }

}

/// Type of deployment
///
/// Can be Dockerfile, DockerCompose (more yet to come)
#[derive(Serialize, Deserialize, Debug)]
pub enum DeploymentType {
    Dockerfile,
    DockerCompose,
}

/// Server Type
///
/// Can be Local / Remote (ipv6)
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerType {
    Local,
    Remote(String),
}

/// Status of deployment
///
/// This is handled internally to keep a track of the deployment
#[derive(Serialize, Deserialize, Debug)]
pub enum DeploymentStatus {
    Idle,
    Starting,
    Running,
    Stopped,
}
