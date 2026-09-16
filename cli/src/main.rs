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
        models::deployment::{Deployment, ServerType},
        runner,
        utils::generate_deployment_name,
    },
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup => commands::setup::run()?,
        Commands::Build { name, file } => {
            let name = name.unwrap_or_else(generate_deployment_name);
            let deployment = Deployment::new(name, file, String::new(), ServerType::Local)?;
            runner::build(&deployment)?;
        }
    }

    Ok(())
}
