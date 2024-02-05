use std::ops::{Index, IndexMut};

use glium::{texture::RawImage2d, Display, Texture2d};

pub struct CPUBuffer {
    data: Vec<[f32; 3]>,
    pub width: u32,
    pub height: u32,
}

impl CPUBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let len = width * height;
        let data = vec![[0.; 3]; len as usize];
        CPUBuffer {
            data: data,
            width: width,
            height: height,
        }
    }

    fn ravel(&self) -> Vec<u8> {
        self.data
            .iter()
            .flatten()
            .map(|x| (x * 255.) as u8)
            .collect()
    }

    pub fn as_texture(&self, display: &Display) -> Texture2d {
        let raw_image = RawImage2d::from_raw_rgb_reversed(&self.ravel(), (self.width, self.height));

        let dest_texture = Texture2d::new(display, raw_image).unwrap();

        dest_texture
    }
}

impl Index<(usize, usize)> for CPUBuffer {
    type Output = [f32; 3];

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + index.1 * (self.width as usize)]
    }
}

impl IndexMut<(usize, usize)> for CPUBuffer {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + index.1 * (self.width as usize)]
    }
}
