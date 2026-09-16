use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::models::deployment::{Deployment, DeploymentType};

/// Execute the build step for a deployment
pub fn build(deployment: &Deployment) -> Result<()> {
    println!(
        "[justdeploy] Building '{}' ({:?})...",
        deployment.name, deployment.deployment_type
    );

    match &deployment.deployment_type {
        DeploymentType::Dockerfile => build_dockerfile(deployment),
        DeploymentType::DockerCompose => build_compose(deployment),
    }
}

fn build_dockerfile(deployment: &Deployment) -> Result<()> {
    let file_path = &deployment.file_path;

    let context_path = file_path
        .parent()
        .context("Could not determine build context: Dockerfile has no parent directory")?;

    let file_str = file_path
        .to_str()
        .context("Dockerfile path contains invalid UTF-8")?;

    let context_str = context_path
        .to_str()
        .context("Build context path contains invalid UTF-8")?;

    let mut cmd = Command::new("docker");
    cmd.args(["build", "-f", file_str]);

    if !deployment.name.is_empty() {
        cmd.args(["-t", &deployment.name]);
    }

    cmd.arg(context_str)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd
        .spawn()
        .context("Failed to spawn 'docker build'. Is Docker installed and running?")?
        .wait()
        .context("Failed to wait for 'docker build' process")?;

    if !status.success() {
        bail!(
            "Docker build failed (exit code: {})",
            status.code().map_or_else(|| "unknown".to_string(), |c| c.to_string())
        );
    }

    println!("[justdeploy] '{}' built successfully!", deployment.name);
    Ok(())
}

fn build_compose(_deployment: &Deployment) -> Result<()> {
    bail!("Docker Compose support is not yet implemented")
}
