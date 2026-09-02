use std::{io, panic};

use color_eyre::eyre::Result;
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{app::App, event::EventHandler, ui};

pub type CrosstermTerminal = Terminal<CrosstermBackend<std::io::Stderr>>;

/// TUI Representation
pub struct Tui {
    /// Terminal interface
    terminal: CrosstermTerminal,
    /// Terminal event handler
    pub events: EventHandler,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initialises the terminal interface
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // custom panic hook to reset terminal props
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    /// Resets the terminal
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }


    /// Exit the terminal interface
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    /// Draw the terminal by rendering the widgets
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }
}
