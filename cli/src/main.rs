/// Application
pub mod app;

/// Terminal events handler
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use color_eyre::{eyre::Result};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{app::App, event::{Event, EventHandler}, tui::Tui, update::update};

fn main() -> Result<()> {
    // Create application
    let mut app = App::new();

    // Initialise TUI
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start main loop
    while !app.exit {
        // Render the UI
        tui.draw(&mut app)?;
        // Handle events
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => { update(&mut app, key_event);}
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    } 

    // Exit UI
    tui.exit()?;

    Ok(())
}

