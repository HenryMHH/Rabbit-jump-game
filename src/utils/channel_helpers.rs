use std::sync::mpsc;

pub enum CustomEvent {
    Input(crossterm::event::KeyEvent),
    Resize(u16, u16),
}

pub fn handle_input_events(tx: mpsc::Sender<CustomEvent>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => {
                tx.send(CustomEvent::Input(key_event)).unwrap()
            }
            crossterm::event::Event::Resize(width, height) => {
                tx.send(CustomEvent::Resize(width, height)).unwrap();
            }
            _ => {}
        }
    }
}
