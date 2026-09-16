/// Application
pub mod app;

/// Terminal events handler
pub mod event;
pub mod models;
pub mod tui;
pub mod ui;
pub mod update;
pub mod utils;

use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::App,
    event::{Event, EventHandler},
    models::deployment::{Deployment, DeploymentType, ServerType},
    tui::Tui,
    update::update,
    utils::generate_deployment_name,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    file: PathBuf,
    name: Option<String>,
}

fn main() -> Result<()> {
    // ------ Clap CLI Stuff ------
    let cli = Cli::parse();

    let deployment_name: String;

    let file_path = cli.file;

    if let Some(name) = cli.name {
        deployment_name = name;
    } else {
        deployment_name = generate_deployment_name();
    }


    // Creating new deployment
    let deployment = Deployment::new(
        deployment_name,
        file_path,
        "".to_string(),
        ServerType::Local,
    );

    println!("Deployment Config: {:?}", deployment);

    deployment.build();

    // ------ TUI Stuff ------
    // Create application
    let mut app = App::new();

    // Initialise TUI
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    // tui.enter()?;
    //
    // // Start main loop
    // while !app.exit {
    //     // Render the UI
    //     tui.draw(&mut app)?;
    //     // Handle events
    //     match tui.events.next()? {
    //         Event::Tick => {}
    //         Event::Key(key_event) => { update(&mut app, key_event);}
    //         Event::Mouse(_) => {}
    //         Event::Resize(_, _) => {}
    //     }
    // }
    //
    // // Exit UI
    // tui.exit()?;

    Ok(())
}
