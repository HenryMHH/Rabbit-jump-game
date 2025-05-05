use rand::random;
use ratatui::layout::Rect;

const SHORT_CACTUS_ASCII: &str = r#"#
# #
###
  #
"#;

const TALL_CACTUS_ASCII: &str = r#"    #
#   # #
# # ###
### #
  # #
"#;

const DEFAULT_DELTA_X: isize = 1;

pub enum CactusType<'a> {
    Short(ShortCactus<'a>),
    Tall(TallCactus<'a>),
}

impl<'a> CactusType<'a> {
    pub fn update_x(&mut self, delta: isize) {
        match self {
            CactusType::Short(s) => s.update_x(delta),
            CactusType::Tall(t) => t.update_x(delta),
        }
    }

    pub fn get_x(&self) -> u16 {
        match self {
            CactusType::Short(s) => s.get_all_attr().0,
            CactusType::Tall(t) => t.get_all_attr().0,
        }
    }

    pub fn get_all_attr(&'a self) -> (u16, u16, u16, &'a str) {
        match self {
            CactusType::Short(s) => s.get_all_attr(),
            CactusType::Tall(t) => t.get_all_attr(),
        }
    }
}

pub trait Obstacle<'a> {
    fn new(start_x: u16) -> Self;
    fn update_x(&mut self, delta_x: isize);
    fn get_all_attr(&'a self) -> (u16, u16, u16, &'a str);
}

pub struct ShortCactus<'a> {
    x: u16,
    width: u16,
    height: u16,
    art: &'a str,
}

impl<'a> Obstacle<'a> for ShortCactus<'a> {
    fn new(start_x: u16) -> Self {
        let art = SHORT_CACTUS_ASCII;
        let width = 3 as u16;
        let height = 4 as u16;
        Self {
            x: start_x - width,
            width,
            height,
            art,
        }
    }

    fn update_x(&mut self, delta_x: isize) {
        self.x = self.x.saturating_sub(delta_x as u16);
    }

    fn get_all_attr(&'a self) -> (u16, u16, u16, &'a str) {
        (self.x, self.width, self.height, self.art)
    }
}

pub struct TallCactus<'a> {
    x: u16,
    width: u16,
    height: u16,
    art: &'a str,
}

impl<'a> Obstacle<'a> for TallCactus<'a> {
    fn new(start_x: u16) -> Self {
        let art = TALL_CACTUS_ASCII;
        let width = 7 as u16;
        let height = 5 as u16;
        Self {
            x: start_x - width,
            width,
            height,
            art,
        }
    }

    fn update_x(&mut self, delta_x: isize) {
        self.x = self.x.saturating_sub(delta_x as u16);
    }

    fn get_all_attr(&'a self) -> (u16, u16, u16, &'a str) {
        (self.x, self.width, self.height, self.art)
    }
}
pub struct ObstacleVec<'a> {
    obstacles: Vec<CactusType<'a>>,
}

impl<'a> ObstacleVec<'a> {
    pub fn new() -> Self {
        Self { obstacles: vec![] }
    }

    pub fn get_obstacles(&self) -> &Vec<CactusType<'a>> {
        &self.obstacles
    }

    pub fn add_new_obstacle(&mut self, frame_rect: Rect) {
        let new_obstacle = Self::get_new_obstacle(frame_rect.width);

        self.obstacles.push(new_obstacle);
    }

    fn get_new_obstacle(frame_width: u16) -> CactusType<'a> {
        let cactus = match random::<u8>() % 2 {
            0 => CactusType::Short(ShortCactus::new(frame_width)),
            _ => CactusType::Tall(TallCactus::new(frame_width)),
        };

        cactus
    }

    pub fn remove_obstacle(&mut self) {
        if !self.obstacles.is_empty() {
            self.obstacles.retain(|cactus| cactus.get_x() > 0);
        }
    }

    pub fn update_obstacles(&mut self, delta_x: Option<isize>) {
        let delta_x = delta_x.unwrap_or(DEFAULT_DELTA_X);

        for cactus in self.obstacles.iter_mut() {
            cactus.update_x(delta_x);
        }
    }
}
