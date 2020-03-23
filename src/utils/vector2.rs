#[derive(Debug, Clone)]
pub struct Vector2u {
    pub x: usize,
    pub y: usize
}


impl Vector2u {
    pub fn new(x: usize, y: usize) -> Vector2u {
        Vector2u { x, y }
    }
}
