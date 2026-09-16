use clap::{Parser, Subcommand};

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
    /// Build the deployment using the project's jd.json config
    Build,
}
