use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// JustDeploy — Deployments made easy
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new JustDeploy configuration (jd.json)
    Setup,
    /// Build the deployment from a Dockerfile or docker-compose file
    Build {
        /// Deployment name (auto-generated if omitted)
        name: Option<String>,
        /// Path to a Dockerfile or docker-compose.yml
        #[arg(short, long)]
        file: PathBuf,
    },
}
