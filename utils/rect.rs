use ratatui::prelude::*;

pub const MAX_FRAME_SIZE: (u16, u16) = (100, 21);

pub fn get_default_frame_rect(terminal: &DefaultTerminal) -> Rect {
    let terminal_size = terminal.size().unwrap();

    let f_w = if terminal_size.width > MAX_FRAME_SIZE.0 {
        MAX_FRAME_SIZE.0
    } else {
        terminal_size.width
    };

    let f_h = if terminal_size.height > MAX_FRAME_SIZE.1 {
        MAX_FRAME_SIZE.1
    } else {
        terminal_size.height
    };

    Rect::new(0, 0, f_w, f_h)
}
