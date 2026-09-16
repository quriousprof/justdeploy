use std::{sync::mpsc, thread, time::{Duration, Instant}};

use anyhow::Result;
use crossterm::event::{self, KeyEvent, MouseEvent, Event as CrosstermEvent};

/// Terminal events
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal Tick
    Tick,
    /// Key Press,
    Key(KeyEvent),
    /// Mouse click/scroll
    Mouse(MouseEvent),
    /// Terminal resize
    Resize(u16, u16),
}


/// Terminal event handler
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`]
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => Ok(()),
                        }
                        .expect("failed to send terminal event")
                    }

                    // send tick
                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self { sender, receiver, handler }
    }

    /// Receive the next event from the handler thread
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
