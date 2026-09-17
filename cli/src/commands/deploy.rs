use std::env;

use anyhow::Result;

use crate::core::{
    models::{
        config::JdConfig,
        deployment::{Deployment, ServerType},
    },
    registry::Registry,
    runner,
};

pub fn run(down: bool) -> Result<()> {
    let config_path = env::current_dir()?.join("jd.json");
    let config = JdConfig::load()?;

    let deployment = Deployment::new(
        config.name,
        config.file_path,
        String::new(),
        ServerType::Local,
    )?;

    if down {
        runner::stop(&deployment)?;
    } else {
        runner::deploy(&deployment)?;
        let mut registry = Registry::load()?;
        registry.mark_deployed(&config_path);
        registry.save()?;
    }

    Ok(())
}
