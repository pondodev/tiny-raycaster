use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::path::Path;

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl GameMap {
    pub fn new( file_path: &str ) -> Self {
        let path = Path::new( file_path );
        let file = File::open( path ).unwrap();
        let mut reader = BufReader::new( file );

        // read in header information
        let mut w_str = String::new();
        let mut h_str = String::new();
        reader.read_line( &mut w_str ).unwrap();
        reader.read_line( &mut h_str ).unwrap();
        w_str.retain( |c| !c.is_whitespace() ); // strip whitespace
        h_str.retain( |c| !c.is_whitespace() );
        let width = w_str.parse::<usize>().unwrap();
        let height = h_str.parse::<usize>().unwrap();

        // read in tiles
        let mut tiles = vec![ Tile::Floor; width * height ];
        let lines = reader.lines();
        let mut x = 0;
        let mut y = 0;
        for line in lines {
            for c in line.unwrap().chars() {
                match c {
                    '_' => (), // floor
                    '#' => tiles[ x + y * width ] = Tile::Wall, // wall
                    _   => println!( "unrecognised tile: {}", c ),
                }

                x += 1;
            }

            y += 1;
            x = 0;
        }

        GameMap { width, height, tiles }
    }
}