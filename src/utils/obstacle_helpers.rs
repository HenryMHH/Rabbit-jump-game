use crate::core::{CactusType, Obstacle, ShortCactus, TallCactus};
use rand::random;

pub fn get_new_obstacle<'a>(frame_width: u16) -> CactusType<'a> {
    let cactus = match random::<u8>() % 2 {
        0 => CactusType::Short(ShortCactus::new(frame_width)),
        _ => CactusType::Tall(TallCactus::new(frame_width)),
    };
    cactus
}
