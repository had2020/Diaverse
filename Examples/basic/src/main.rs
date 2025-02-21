use Diaverse::*;

fn main() {
    //window_init()
    let stored_world: Storage = Storage::create_new(Shape { x: 2, y: 2 }, "world/test.txt");
    generate_chunk(stored_world);
    window_init();
}
