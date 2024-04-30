mod color;
mod image_map;
use std::cell::Cell;

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

    for ((x, y), _pixel) in image_map.iter() {
        let rand_color = color_palette
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        let above_y = y.clone() as i32 - 1;

        let mut color_options = vec![rand_color];
        if let Some(last_color) = last_color {
            color_options.push(last_color.clone());
        }
        if let Some(above_color) = image_map.get(x, &(above_y as usize)) {
            color_options.push(above_color);
        }

        let color = color_options
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();

        last_color = Some(color.clone());
        _pixel.set(color.clone());
    }
    return image_map;
}
