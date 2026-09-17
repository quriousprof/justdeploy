use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use super::{logger, models::deployment::{Deployment, DeploymentType}};

/// Execute the build step for a deployment
pub fn build(deployment: &Deployment) -> Result<()> {
    logger::info(&format!("Building '{}'...", deployment.name));

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

    logger::success(&format!("'{}' built successfully!", deployment.name));
    Ok(())
}

fn build_compose(deployment: &Deployment) -> Result<()> {
    let file_str = deployment
        .file_path
        .to_str()
        .context("Compose file path contains invalid UTF-8")?;

    let status = Command::new("docker")
        .args(["compose", "-f", file_str, "-p", &deployment.name, "build"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn 'docker compose'. Is Docker installed and running?")?
        .wait()
        .context("Failed to wait for 'docker compose build' process")?;

    if !status.success() {
        bail!(
            "docker compose build failed (exit code: {})",
            status.code().map_or_else(|| "unknown".to_string(), |c| c.to_string())
        );
    }

    logger::success(&format!("'{}' built successfully!", deployment.name));
    Ok(())
}
