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

pub enum CactusType<'a> {
    Short(ShortCactus<'a>),
    Tall(TallCactus<'a>),
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
