/// Central Dispatcher
#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,
}

impl App {
    /// Construct a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal
    pub fn tick(&self) {}

    /// Set quit to true to quit the application
    pub fn quit(&mut self) {
        self.exit = true;
    }

}
