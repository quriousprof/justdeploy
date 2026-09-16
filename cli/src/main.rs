pub mod app;
pub mod cli;
pub mod event;
pub mod logger;
pub mod models;
pub mod runner;
pub mod tui;
pub mod ui;
pub mod update;
pub mod utils;

use anyhow::Result;
use clap::Parser;

use crate::{
    cli::Cli,
    models::deployment::{Deployment, ServerType},
    utils::generate_deployment_name,
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let name = cli.name.unwrap_or_else(generate_deployment_name);

    let deployment = Deployment::new(name, cli.file, String::new(), ServerType::Local)?;
    runner::build(&deployment)?;

    Ok(())
}
