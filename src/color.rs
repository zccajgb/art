pub struct Pallete {}
impl Pallete {
    pub const WHITE: Color = [234, 222, 218];
    pub const BLACK: Color = [46, 40, 42];
    pub const RED: Color = [205, 83, 52];
    pub const BLUE: Color = [115, 137, 174];
}

impl Pallete {
    pub fn get_pallette() -> Vec<Color> {
        vec![Pallete::WHITE, Pallete::BLACK, Pallete::RED, Pallete::BLUE]
    }

    #[allow(dead_code)]
    pub fn get_pallette_as_pixels() -> Vec<image::Rgb<u8>> {
        Pallete::get_pallette()
            .iter()
            .map(|color| image::Rgb::<u8>(*color))
            .collect()
    }
}

pub type Color = [u8; 3];
