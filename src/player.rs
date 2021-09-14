pub struct Player {
    pub x: usize,
    pub y: usize,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Player { x, y }
    }
}