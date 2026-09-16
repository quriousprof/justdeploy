pub mod app;
pub mod cli;
pub mod commands;
pub mod core;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;
use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    core::{
        models::{
            config::JdConfig,
            deployment::{Deployment, ServerType},
        },
        runner,
    },
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup => commands::setup::run()?,
        Commands::Build => {
            let config = JdConfig::load()?;
            let deployment = Deployment::new(
                config.name,
                config.file_path,
                String::new(),
                ServerType::Local,
            )?;
            runner::build(&deployment)?;
        }
    }

    Ok(())
}
