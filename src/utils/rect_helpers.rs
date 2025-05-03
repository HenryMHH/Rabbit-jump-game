use crate::constants::size::MAX_FRAME_SIZE;
use ratatui::layout::Rect;

pub fn get_default_frame_rect(current_width: u16, current_height: u16) -> Rect {
    let f_w = if current_width > MAX_FRAME_SIZE.0 {
        MAX_FRAME_SIZE.0
    } else {
        current_width
    };

    let f_h = if current_height > MAX_FRAME_SIZE.1 {
        MAX_FRAME_SIZE.1
    } else {
        current_height
    };

    Rect::new(0, 0, f_w, f_h)
}
