pub struct Shape {
    pub x: i32,
    pub y: i32,
}

pub struct Storage {
    pub location: String,
    pub max_chucks_shape: Shape,
}

pub struct Atom {
    pub denisty: f32,
    pub heat: f32,
}

pub struct Chunk {
    pub max_Shape: Shape,
    pub atoms: Vec<Vec<Atom>>,
}
