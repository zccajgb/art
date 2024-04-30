use indexmap::{
    map::{Iter, IterMut},
    IndexMap,
};
use itertools::iproduct;
use log::{trace, warn};
use std::{
    cell::Cell,
    collections::HashMap,
    fmt::{self, Formatter},
};

use crate::color::Color;

#[derive(Clone)]
pub struct ImageMap {
    underlying_map: IndexMap<(usize, usize), Cell<Color>>,
    pub width: usize,
    pub height: usize,
}

impl ImageMap {
    pub fn new(x: usize, y: usize) -> Self {
        let prod = iproduct!(0..x, 0..y);
        let map_iter = prod.into_iter().map(|x| (x, Cell::new([0 as u8, 0, 0])));
        let hash_map: IndexMap<(usize, usize), Cell<Color>> = IndexMap::from_iter(map_iter);
        return ImageMap {
            underlying_map: hash_map,
            width: x,
            height: y,
        };
    }
}

impl ImageMap {
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<image::Rgb<u8>> {
        let color = self.underlying_map.get(&(x as usize, y as usize))?;
        Some(image::Rgb::<u8>(color.get()))
    }
    #[allow(dead_code)]
    pub fn get(&self, x: &usize, y: &usize) -> Option<Color> {
        if x >= &self.width || y >= &self.height {
            warn!("Out of bounds! x: {}, y: {}", x, y);
            return None;
        }
        self.underlying_map.get(&(*x, *y)).map(|x| x.get())
    }
    #[allow(dead_code)]
    pub fn from_iter<'a, I>(iter: I) -> Self
    where
        I: Iterator<Item = ((usize, usize), Cell<Color>)>,
    {
        let hash_map: IndexMap<(usize, usize), Cell<Color>> = IndexMap::from_iter(iter);
        let (width, height) = hash_map
            .keys()
            .fold((0, 0), |acc, (x, y)| (acc.0.max(*x), acc.1.max(*y)));
        return ImageMap {
            underlying_map: hash_map,
            width: width + 1,
            height: height + 1,
        };
    }

    // pub fn from_vec(vec: Vec<((usize, usize), Cell<Color>)>) -> Self {
    //     let mut hash_map: IndexMap<(usize, usize), Cell<Color>> = IndexMap::new();
    //     for ((x, y), c) in vec.iter() {
    //         hash_map.insert((*x, *y), *c);
    //     }
    //     let (width, height) = hash_map
    //         .keys()
    //         .fold((0, 0), |acc, (x, y)| (acc.0.max(*x), acc.1.max(*y)));
    //     return ImageMap {
    //         underlying_map: hash_map,
    //         width: width + 1,
    //         height: height + 1,
    //     };
    // }
}

impl ImageMap {
    pub fn iter(&self) -> Iter<'_, (usize, usize), Cell<Color>> {
        self.underlying_map.iter()
    }
    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> IterMut<'_, (usize, usize), Cell<Color>> {
        self.underlying_map.iter_mut()
    }
}

impl fmt::Debug for ImageMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ImageMap {{ width: {}, height: {} }}. Map: {:?}",
            self.width, self.height, self.underlying_map
        )
    }
}
