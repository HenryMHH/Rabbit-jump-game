mod constants;
mod core;
mod utils;

use anyhow::Result;

use constants::game::GameState;
use core::{App, Ground, Rabbit};
use std::{
    sync::mpsc::{self},
    thread,
};
use utils::{CustomEvent, get_default_frame_rect, handle_input_events};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<CustomEvent>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let terminal_size = terminal.size()?;

    let terminal_rect = get_default_frame_rect(terminal_size.width, terminal_size.height);

    let mut app = App {
        frame_rect: terminal_rect,
        state: GameState::Running,
        ground: Ground::new("-^-.-^.".to_string()),
        rabbit: Rabbit::new(),
        obstacles: vec![],
    };

    app.run(&mut terminal, event_rx)?;

    ratatui::restore();
    Ok(())
}
