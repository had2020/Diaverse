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
    pub loaded_chucks: Vec<Chunk>,
}

pub fn generate_chunk(storage: Storage) {
    let mut chunk: Vec<Vec<Atom>> = vec![];
    let blank = 1.0;
    for row in 0..storage.max_chucks_shape.x {
        chunk.push(vec![]);
        for col in 0..storage.max_chucks_shape.y {
            chunk[row as usize].push(Atom {
                mass: blank,
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
            loaded_chucks: vec![],
        }
    }
}

pub struct Atom {
    pub mass: f32,
    pub density: f32,
    pub heat: f32,
    pub vibration: f32,
}

pub struct Chunk {
    pub max_shape: Shape,
    pub atoms: Vec<Vec<Atom>>,
}

pub struct Window_session {
    pub width: usize,
    pub height: usize,
    pub window: Window,
    pub buffer: Vec<u32>,
}

use minifb::{Key, Window, WindowOptions};

/// Good size is 800, 600
impl Window_session {
    pub fn init(width: usize, height: usize, name: &str) -> Self {
        let window = Window::new(name, width, height, WindowOptions::default())
            .expect("Unable to open window");

        //1D array of pixels
        let buffer: Vec<u32> = vec![0x00; width * height];

        Window_session {
            width: width,
            height: height,
            window: window,
            buffer: buffer,
        }
    }
}

pub fn window_init(width: usize, height: usize, name: &str) -> (Window, Vec<u32>) {
    let window =
        Window::new(name, width, height, WindowOptions::default()).expect("Unable to open window");

    //1D array of pixels
    let buffer: Vec<u32> = vec![0x00; width * height];

    /*
    // draw red pixel at 10,10 for test
    let index = (10 * width + 10) as usize;
    buffer[index] = 0xFF0000FF; // works blue, TODO color for float
    */

    /*
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
    */
    (window, buffer)
}
