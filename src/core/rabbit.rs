pub const RABBIT_ART: &str = r#" (\(\
 (• •)
o/っ っ"#;

pub const RABBIT_WIDTH: u16 = 9; // ART 的寬度
pub const RABBIT_HEIGHT: f32 = 3.0; // ART 的高度
pub const RABBIT_X_POS: u16 = 5; // 兔子固定的 X 座標
pub const JUMP_STRENGTH: f32 = 1.0; // 跳躍力
pub const GRAVITY: f32 = 0.06; // 重力

pub struct Rabbit {
    pub x: u16,
    y: f32,
    velocity: f32,
}

impl Rabbit {
    pub fn new() -> Self {
        Self {
            x: RABBIT_X_POS,
            y: 0.0,
            velocity: 0.0,
        }
    }

    pub fn update_physics(&mut self) {
        self.y += self.velocity;
        self.velocity -= GRAVITY;

        if self.y <= 0.0 {
            self.y = 0.0;
            self.velocity = 0.0;
        }
    }

    pub fn jump(&mut self) {
        if self.y <= 0.0 && self.velocity <= 0.0 {
            self.velocity = JUMP_STRENGTH as f32;
        }
    }

    pub fn get_top_y(&self, ground_render_y: f32) -> u16 {
        (ground_render_y - RABBIT_HEIGHT - self.y).max(0.0) as u16
    }
}
