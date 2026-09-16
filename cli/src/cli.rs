use std::path::PathBuf;

use clap::Parser;

/// JustDeploy — Deployments made easy
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to a Dockerfile or docker-compose.yml
    #[arg(short, long)]
    pub file: PathBuf,

    /// Deployment name (auto-generated if omitted)
    pub name: Option<String>,
}
