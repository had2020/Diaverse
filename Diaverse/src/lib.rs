pub struct Shape {
    pub x: i32,
    pub y: i32,
}

pub struct Storage {
    pub location: &str,
    pub max_chucks_shape: Shape,
}

pub struct Chunk {
    pub max_Shape: Shape,
    pub atoms: Vec<Vec<f64>>,
}
