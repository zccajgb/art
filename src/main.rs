mod color;
mod image_map;
use color::{Color, Pallete};
use image::{ImageBuffer, Rgb};
use image_map::ImageMap;
use log::{debug, error, trace};
use pretty_env_logger;
use rand::seq::SliceRandom;

fn main() {
    pretty_env_logger::init();
    let x: usize = 10;
    let y: usize = 10;
    let mut image_map = ImageMap::new(x, y);
    image_map = create_image(image_map);
    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(x as u32, y as u32);
    image.enumerate_pixels_mut().for_each(|(x, y, mut _pixel)| {
        *_pixel = image_map.get_pixel(x, y).unwrap();
    });
    image.save("test.png").unwrap();
}

fn create_image(image_map: ImageMap) -> ImageMap {
    let color_palette: Vec<Color> = Pallete::get_pallette();
    let mut last_color: Option<Color> = None;
    let above_color: Option<Color> = None;

    let mut new_map: Vec<((usize, usize), Color)> = Vec::new();

    image_map.iter().for_each(|((x, y), _pixel)| {
        let rand_color = color_palette
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        let above_y = y.clone() as i32 - 1;
        // let index = y * image_map.width as i32 + x.clone() as i32;
        let mut color_options = vec![rand_color];
        if let Some(last_color) = last_color {
            color_options.push(last_color.clone());
        }
        if let Some(above_color) = new_map
            .iter()
            .find(|((x, y), _color)| y == &(above_y as usize) && x == x)
            .map(|((_x, _y), color)| color)
        {
            color_options.push(above_color.clone());
        }

        let color = color_options
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        if color == [0, 0, 0] {
            debug!("Black! at x: {}, y: {}", x, y);
            debug!("Above color: {:?}", above_color);
            debug!("Prev color: {:?}", last_color);
            debug!("Rand color: {:?}", rand_color);
            debug!("Color options: {:?}", color_options);
            panic!();
        }
        last_color = Some(color.clone());
        new_map.push(((x.clone(), y.clone()), color));
    });
    let image_map = ImageMap::from_vec(new_map);
    return image_map;
}
