// Diaverse
// By: Hadrian Lazic

use std::fs::File;

#[derive(Clone, Debug)]
pub struct Shape {
    pub x: i32,
    pub y: i32,
}

pub struct Storage {
    pub location: String,
    pub max_chucks_shape: Shape,
}

pub fn generate_chunk(storage: Storage) {
    let mut chunk: Vec<Vec<Atom>> = vec![];
    let blank = 1.0;
    for row in 0..storage.max_chucks_shape.x {
        for col in 0..storage.max_chucks_shape.y {
            chunk[row as usize].push(Atom {
                density: blank,
                heat: blank,
                vibration: blank,
            });
        }
    }
}

impl Storage {
    pub fn create_new(chucks_shape: Shape, save_file_path: &str) -> Self {
        File::create(save_file_path).unwrap();
        Storage {
            location: save_file_path.to_string(),
            max_chucks_shape: chucks_shape,
        }
    }
}

pub struct Atom {
    pub density: f32,
    pub heat: f32,
    pub vibration: f32,
}

pub struct Chunk {
    pub max_shape: Shape,
    pub atoms: Vec<Vec<Atom>>,
}

use minifb::{Key, Window, WindowOptions};

pub fn window_init() {
    let width = 800;
    let height = 600;

    let mut window = Window::new("Test", width, height, WindowOptions::default())
        .expect("Unable to open window");

    //1D array of pixels
    let mut buffer: Vec<u32> = vec![0x00FF00FF; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
