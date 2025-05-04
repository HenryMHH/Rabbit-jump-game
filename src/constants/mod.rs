pub mod size {
    pub const MAX_FRAME_SIZE: (u16, u16) = (100, 21);
}

pub mod timing {
    use std::time::Duration;

    pub const TICK_RATE: Duration = Duration::from_millis(16);
    pub const GROUND_SCROLL_SPEED: Duration = Duration::from_millis(16);
}

pub mod game {
    pub const GROUND_SHIFT: u16 = 3;

    #[derive(Debug, PartialEq, Eq)]
    pub enum GameState {
        Running,
        GameOver,
        Quit,
    }
}
