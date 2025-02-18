pub struct Shape {
    pub x: i32,
    pub y: i32,
}

pub struct Storage {
    pub location: String,
    pub max_chucks_shape: Shape,
}

impl Storage {
    pub fn create_new(chucks_shape: Shape, filename: String) -> Self {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::create(&filename).unwrap();
        file.write_all("test".as_bytes()).unwrap();
        Storage {
            location: "test".to_string(),
            max_chucks_shape: chucks_shape,
        }
    }
}

pub struct Atom {
    pub denisty: f32,
    pub heat: f32,
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
