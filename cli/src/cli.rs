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
    /// Build and run the deployment on the local machine
    Deploy {
        /// Start the deployment (default)
        #[arg(long, conflicts_with = "down")]
        up: bool,
        /// Stop and remove the running containers
        #[arg(long, conflicts_with = "up")]
        down: bool,
    },
    /// Stream logs for a deployment by name, or for the current directory's project
    Logs {
        /// Name of the deployment (defaults to current directory's jd.json)
        name: Option<String>,
    },
    /// List all registered deployments
    List,
}
