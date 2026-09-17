use colored::Colorize;

use crate::core::{
    logger,
    models::{config::JdConfig, deployment::DeploymentType},
    registry::Registry,
};

pub fn run() -> anyhow::Result<()> {
    let registry = Registry::load()?;

    if registry.deployments.is_empty() {
        logger::info("No deployments registered. Run `jd setup` in a project directory.");
        return Ok(());
    }

    println!();
    for entry in &registry.deployments {
        match JdConfig::load_from(&entry.config_path) {
            Ok(config) => {
                let kind = match config.deployment_type {
                    DeploymentType::Dockerfile => "Dockerfile",
                    DeploymentType::DockerCompose => "Docker Compose",
                };
                let built = entry
                    .last_built_at
                    .map(|t| t.format("%Y-%m-%d %H:%M UTC").to_string())
                    .unwrap_or_else(|| "never".to_string());

                println!("  {} {}", "▸".cyan().bold(), config.name.bold());
                println!("    project   {}", config.project_dir.display());
                println!("    type      {}", kind);
                println!("    config    {}", entry.config_path.display());
                println!("    built     {}", built);
                println!();
            }
            Err(_) => {
                logger::warn(&format!(
                    "Config missing or unreadable: {}",
                    entry.config_path.display()
                ));
            }
        }
    }

    Ok(())
}
