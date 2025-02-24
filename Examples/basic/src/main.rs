use minifb::Key;
use Diaverse::*;

fn main() {
    //let mut (window, buffer) = window_init(800, 600, "Diaverse");
    let mut win = Window_session::init(800, 600, "Diaverse");
    let mut stored_world: World = World::create_new(Shape { x: 10, y: 10 }, "world/test.txt");

    for chunk in 0..2 {
        generate_chunk(&mut stored_world);
    }

    //draw_pixel(&mut win, position!(1, 50), 0xFF0000FF);
    apply_heat(&mut stored_world, 100.0, 1, position!(5, 5));

    // window frame time render loop
    while win.window.is_open() && !win.window.is_key_down(Key::Escape) {
        if win.window.is_key_down(Key::Up) {
            win.observer_position.y += 1;
        }
        render_chunks(&stored_world, &mut win);

        win.window
            .update_with_buffer(&win.buffer, win.width, win.height)
            .unwrap();
    }
}
