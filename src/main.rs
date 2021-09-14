#![allow(dead_code)]

pub mod map;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use map::*;

const WINDOW_WIDTH: usize = 512;
const WINDOW_HEIGHT: usize = 512;
const FRAMEBUFFER_SIZE: usize = WINDOW_WIDTH * WINDOW_HEIGHT;

fn main() {
    // colours stored as hex RGBA
    let mut framebuffer: [ u32; FRAMEBUFFER_SIZE ]
        = [ 0x00000000; FRAMEBUFFER_SIZE ];

    // temp pretty colour background to show coordinate space
    for y in 0..WINDOW_HEIGHT {
        for x in 0..WINDOW_WIDTH {
            let r = (255 * x / WINDOW_WIDTH) as u8;
            let g = (255 * y / WINDOW_HEIGHT) as u8;
            let b = 0;
            let a = 255;

            framebuffer[ x + y * WINDOW_WIDTH ] = encode_color(r, g, b, a)
        }
    }

    let map = GameMap::new( "map.txt" );
    draw_tiles(&mut framebuffer, &map);

    buffer_to_image( &framebuffer );
}

fn decode_color( hex: u32 ) -> (u8, u8, u8, u8) {
    let r = ((hex & 0xFF000000) >> 24) as u8;
    let g = ((hex & 0x00FF0000) >> 16) as u8;
    let b = ((hex & 0x0000FF00) >> 8) as u8;
    let a = (hex & 0x000000FF) as u8;

    (r, g, b, a)
}

fn encode_color( r: u8, g: u8, b: u8, a: u8 ) -> u32 {
    let mut col: u32 = 0;
    col |= (r as u32) << 24;
    col |= (g as u32) << 16;
    col |= (b as u32) << 8;
    col |= a as u32;

    col
}

fn buffer_to_image( framebuffer: &[u32] ) {
    let path = Path::new( "framebuffer.ppm" );
    let mut file = match File::create( path ) {
        Ok(f)  => f,
        Err(e) => panic!("failed to open file: {}", e),
    };
    file.write_all( b"P3\n" ).unwrap();
    file.write_all( format!( "{} {}\n", WINDOW_WIDTH, WINDOW_HEIGHT ).as_bytes() )
        .unwrap();
    file.write_all( b"255\n" ).unwrap();

    for i in 0..framebuffer.len() {
        let (r, g, b, _) = decode_color( framebuffer[ i ] );
        file.write_all( format!( "{} {} {}\n", r, g, b ).as_bytes() ).unwrap();
    }
}

fn draw_tiles( buffer: &mut [u32], map: &GameMap ) {
    let tile_width = WINDOW_WIDTH / map.width;
    let tile_height = WINDOW_HEIGHT / map.height;

    for y in 0..map.height {
        for x in 0..map.width {
            match map.tiles[ x + y * map.width ] {
                Tile::Wall  => draw_wall(
                    buffer,
                    x * tile_width,
                    y * tile_height,
                    tile_width,
                    tile_height ),
                Tile::Floor => (),
            }
        }
    }
}

fn draw_wall( buffer: &mut [u32], x: usize, y: usize, w: usize, h: usize ) {
    for i in 0..w {
        for j in 0..h {
            let buf_x = x + i;
            let buf_y = y + j;
            buffer[ buf_x + buf_y * WINDOW_WIDTH ] = 0xFFFFFFFF;
        }
    }
}
