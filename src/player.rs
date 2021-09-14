pub struct Player {
    pub x: f32,
    pub y: f32,
    pub size: usize,
}

impl Player {
    pub fn new( x: f32, y: f32, size: usize ) -> Self {
        Player { size, x, y }
    }

    pub fn get_world_pos( &self, tile_width: usize, tile_height: usize ) -> (usize, usize) {
        let x = self.x * tile_width as f32;
        let y = self.y * tile_height as f32;

        println!("x: {} {}", self.x, x);
        println!("y: {} {}", self.y, y);

        (x as usize, y as usize)
    }
}