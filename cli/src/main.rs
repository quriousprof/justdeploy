pub mod app;
pub mod cli;
pub mod core;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;
use clap::Parser;

use crate::{
    cli::Cli,
    core::{
        models::deployment::{Deployment, ServerType},
        runner,
        utils::generate_deployment_name,
    },
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let name = cli.name.unwrap_or_else(generate_deployment_name);

    let deployment = Deployment::new(name, cli.file, String::new(), ServerType::Local)?;
    runner::build(&deployment)?;

    Ok(())
}
