use std::path::PathBuf;

use anyhow::{Result, bail};
use random_word::{Lang, get};

use crate::models::deployment::DeploymentType;

/// Generate a random deployment name
pub fn generate_deployment_name() -> String {
    format!("{}-{}", get(Lang::En), get(Lang::En)).to_string()
}


// Parse a given file path and check for its contents
pub fn parse_file(file_path: &PathBuf) -> Result<DeploymentType> {
    
    // checking if file_path is valid
    if !(file_path.exists()) {
        bail!("File {} doesn't exist", file_path.to_str().unwrap())
    }

    // deducing the file type
    match file_path.extension() {
        Some(ext) => {
            if ext == "yml" || ext == "yaml" {

                if file_path.file_name().unwrap().to_str().unwrap().contains("compose") {
                    Ok(DeploymentType::DockerCompose)
                } else {
                    bail!("Invalid yml file, does not contain 'compose' in filename")
                }

            } else {
                bail!("Invalid file type.")
            }
        },
        None => {
            if file_path.file_name().unwrap().to_str().unwrap().contains("Dockerfile") {
                Ok(DeploymentType::Dockerfile)
            }
            else {
                bail!("Invalid file type.")
            }
        } 
    }

}
