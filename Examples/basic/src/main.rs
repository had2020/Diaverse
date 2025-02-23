use minifb::Key;
use Diaverse::*;

fn main() {
    //let mut (window, buffer) = window_init(800, 600, "Diaverse");
    let mut win = Window_session::init(800, 600, "Diaverse");
    let mut stored_world: World = World::create_new(Shape { x: 2, y: 2 }, "world/test.txt");

    for chunk in 0..2 {
        generate_chunk(&mut stored_world);
    }

    // draw red pixel at 10,10 for test
    let index = (10 * win.width + 10) as usize;
    win.buffer[index] = 0xFF0000FF; // works blue, TODO color for float

    // window frame time render loop
    while win.window.is_open() && !win.window.is_key_down(Key::Escape) {
        render_chunks(&stored_world);

        win.window
            .update_with_buffer(&win.buffer, win.width, win.height)
            .unwrap();
    }
}
