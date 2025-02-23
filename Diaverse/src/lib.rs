// Diaverse
// By: Hadrian Lazic

use std::fs::File;

#[derive(Clone, Debug)]
pub struct Shape {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct World {
    pub location: String,
    pub max_chucks_shape: Shape,
    pub loaded_chucks: Vec<Chunk>,
}

pub fn generate_chunk(storage: &mut World) {
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
    storage.loaded_chucks.push(Chunk { atoms: chunk });
}

impl World {
    pub fn create_new(chucks_shape: Shape, save_file_path: &str) -> Self {
        //File::create(save_file_path).unwrap();

        match std::fs::create_dir(save_file_path) {
            Ok(_) => println!("Memory dir created ✅"),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!("Using existing file ✅");
                }
            }
        }

        World {
            location: save_file_path.to_string(),
            max_chucks_shape: chucks_shape,
            loaded_chucks: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Atom {
    pub mass: f32,
    pub density: f32,
    pub heat: f32,
    pub vibration: f32,
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub atoms: Vec<Vec<Atom>>,
}

pub struct Window_session {
    pub width: usize,
    pub height: usize,
    pub window: Window,
    pub buffer: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

///draw_pixel(&mut win, position!(10, 10), 0xFF0000FF);
#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr) => {
        Position { x: $x, y: $y }
    };
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

pub fn draw_pixel(win: &mut Window_session, position: Position, color: u32) {
    let index = (position.y * win.width + position.x) as usize;
    win.buffer[index] = color;
}

pub fn render_chunks(stored_world: &World, win: &mut Window_session) {
    let mut offset = Position { x: 0, y: 0 };
    let mut iteration_element_position = Position { x: 0, y: 0 };

    for chunk in stored_world.loaded_chucks.iter() {
        iteration_element_position.x = offset.x;

        for row in chunk.atoms.iter() {
            iteration_element_position.y += 1;

            for col in row.iter() {
                iteration_element_position.x += 1;
                //let x = col.mass * col.density;
                let atom_color = color(col.mass as u8, col.density as u8, col.heat as u8);
                draw_pixel(win, iteration_element_position.clone(), 0xFF0000FF);
            }
            iteration_element_position.x = offset.x;
        }

        iteration_element_position.y = 0;
        offset.x += stored_world.max_chucks_shape.x as usize;
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

pub fn color(red: u8, green: u8, blue: u8) -> u32 {
    (255 << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
}
