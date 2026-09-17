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

/// Build and run the deployment in detached mode
pub fn deploy(deployment: &Deployment) -> Result<()> {
    logger::info(&format!("Deploying '{}'...", deployment.name));

    match &deployment.deployment_type {
        DeploymentType::Dockerfile => deploy_dockerfile(deployment),
        DeploymentType::DockerCompose => deploy_compose(deployment),
    }
}

fn deploy_dockerfile(deployment: &Deployment) -> Result<()> {
    build_dockerfile(deployment)?;

    // Stop and remove existing container if running
    Command::new("docker")
        .args(["rm", "-f", &deployment.name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok();

    let status = Command::new("docker")
        .args(["run", "-d", "--name", &deployment.name, &deployment.name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn 'docker run'")?
        .wait()
        .context("Failed to wait for 'docker run' process")?;

    if !status.success() {
        bail!(
            "docker run failed (exit code: {})",
            status.code().map_or_else(|| "unknown".to_string(), |c| c.to_string())
        );
    }

    logger::success(&format!("'{}' is running.", deployment.name));
    Ok(())
}

fn deploy_compose(deployment: &Deployment) -> Result<()> {
    let file_str = deployment
        .file_path
        .to_str()
        .context("Compose file path contains invalid UTF-8")?;

    let status = Command::new("docker")
        .args([
            "compose", "-f", file_str, "-p", &deployment.name, "up", "--build", "-d",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn 'docker compose up'")?
        .wait()
        .context("Failed to wait for 'docker compose up' process")?;

    if !status.success() {
        bail!(
            "docker compose up failed (exit code: {})",
            status.code().map_or_else(|| "unknown".to_string(), |c| c.to_string())
        );
    }

    logger::success(&format!("'{}' is running.", deployment.name));
    Ok(())
}

/// Stream logs for a running deployment
pub fn logs(deployment: &Deployment) -> Result<()> {
    match &deployment.deployment_type {
        DeploymentType::Dockerfile => logs_dockerfile(deployment),
        DeploymentType::DockerCompose => logs_compose(deployment),
    }
}

fn logs_dockerfile(deployment: &Deployment) -> Result<()> {
    Command::new("docker")
        .args(["logs", "-f", &deployment.name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn 'docker logs'")?
        .wait()
        .context("Failed to wait for 'docker logs'")?;
    Ok(())
}

fn logs_compose(deployment: &Deployment) -> Result<()> {
    let file_str = deployment
        .file_path
        .to_str()
        .context("Compose file path contains invalid UTF-8")?;

    Command::new("docker")
        .args([
            "compose", "-f", file_str, "-p", &deployment.name, "logs", "-f",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn 'docker compose logs'")?
        .wait()
        .context("Failed to wait for 'docker compose logs'")?;
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
