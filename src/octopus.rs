const SHORT_CACTUS_ASCII: &str = r#"#
# #
###
  #
"#;

const TALL_CACTUS_ASCII: &str = r#"#
# #
###
###
  #
"#;

pub enum CactusType<'a> {
    Short(ShortCactus<'a>),
    Tall(TallCactus<'a>),
}

pub trait Cactus<'a> {
    fn new(start_x: u16) -> Self;
    fn update_x(&mut self, delta_x: isize);
    fn get_art(&'a self) -> &'a str;
    fn get_all_attr(&'a self) -> (u16, u16, u16, &'a str);
}

pub struct ShortCactus<'a> {
    x: u16,
    width: u16,
    height: u16,
    art: &'a str,
}

impl<'a> Cactus<'a> for ShortCactus<'a> {
    fn new(start_x: u16) -> Self {
        let art = SHORT_CACTUS_ASCII;
        let width = art.lines().next().unwrap().len() as u16;
        let height = art.lines().count() as u16;
        Self {
            x: start_x,
            width,
            height,
            art,
        }
    }

    fn update_x(&mut self, delta_x: isize) {
        self.x = self.x.saturating_sub(delta_x as u16);
    }

    fn get_art(&self) -> &str {
        &self.art
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

impl<'a> Cactus<'a> for TallCactus<'a> {
    fn new(start_x: u16) -> Self {
        let art = TALL_CACTUS_ASCII;
        let width = art.lines().next().unwrap().len() as u16;
        let height = art.lines().count() as u16;
        Self {
            x: start_x,
            width,
            height,
            art,
        }
    }

    fn update_x(&mut self, delta_x: isize) {
        self.x = self.x.saturating_add(delta_x as u16);
    }

    fn get_art(&self) -> &str {
        &self.art
    }

    fn get_all_attr(&'a self) -> (u16, u16, u16, &'a str) {
        (self.x, self.width, self.height, self.art)
    }
}
