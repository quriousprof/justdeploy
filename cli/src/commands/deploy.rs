use std::{env, path::PathBuf};

use anyhow::Result;

use crate::core::{
    logger,
    models::{
        config::JdConfig,
        deployment::{Deployment, DeploymentType, ServerType},
    },
    registry::Registry,
    runner,
};

pub fn run(down: bool) -> Result<()> {
    let config_path = env::current_dir()?.join("jd.json");
    let config = JdConfig::load()?;
    let deployment_args = config.deployment_args.clone();

    let deployment = Deployment::new(
        config.name,
        config.file_path,
        String::new(),
        ServerType::Local,
    )?;

    if down {
        runner::stop(&deployment)?;
    } else {
        ensure_built(&config_path, &deployment)?;
        runner::deploy(&deployment, &deployment_args)?;
        let mut registry = Registry::load()?;
        registry.mark_deployed(&config_path);
        registry.save()?;
    }

    Ok(())
}

fn ensure_built(config_path: &PathBuf, deployment: &Deployment) -> Result<()> {
    let is_built = match &deployment.deployment_type {
        // For Dockerfile, ask Docker directly — the image may have been removed manually
        DeploymentType::Dockerfile => runner::image_exists(&deployment.name),
        // For Compose, image names aren't predictable, so rely on the registry
        DeploymentType::DockerCompose => Registry::load()?
            .deployments
            .iter()
            .find(|e| e.config_path == *config_path)
            .map(|e| e.last_built_at.is_some())
            .unwrap_or(false),
    };

    if !is_built {
        logger::info("No build found. Running `jd build` first...");
        println!();
        runner::build(deployment)?;
        let mut registry = Registry::load()?;
        registry.mark_built(config_path);
        registry.save()?;
        println!();
    }

    Ok(())
}
