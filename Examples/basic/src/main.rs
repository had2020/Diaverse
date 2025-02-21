use Diaverse::*;

fn main() {
    let mut (window, buffer) = window_init();
    let stored_world: Storage = Storage::create_new(Shape { x: 2, y: 2 }, "world/test.txt");
    generate_chunk(stored_world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
