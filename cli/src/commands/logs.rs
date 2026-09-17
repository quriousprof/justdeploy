use anyhow::{bail, Result};

use crate::core::{
    models::{
        config::JdConfig,
        deployment::{Deployment, ServerType},
    },
    registry::Registry,
    runner,
};

pub fn run(name: &str) -> Result<()> {
    let registry = Registry::load()?;

    for entry in &registry.deployments {
        match JdConfig::load_from(&entry.config_path) {
            Ok(config) if config.name == name => {
                let deployment = Deployment::new(
                    config.name,
                    config.file_path,
                    String::new(),
                    ServerType::Local,
                )?;
                return runner::logs(&deployment);
            }
            _ => continue,
        }
    }

    bail!(
        "No deployment named '{}' found. Run `jd list` to see all deployments.",
        name
    )
}
