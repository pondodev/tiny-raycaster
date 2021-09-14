#![allow(dead_code)]

pub mod map;
pub mod player;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use map::*;
use player::*;
use std::f32::consts::PI;

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 512;
const FRAMEBUFFER_SIZE: usize = WINDOW_WIDTH * WINDOW_HEIGHT;

fn main() {
    // colours stored as hex RGBA
    let mut framebuffer: [ u32; FRAMEBUFFER_SIZE ]
        = [ 0xFFFFFFFF; FRAMEBUFFER_SIZE ];

    let map = GameMap::new( "map.txt" );
    draw_tiles(&mut framebuffer, &map);

    let tile_width = WINDOW_WIDTH / (map.width * 2);
    let tile_height = WINDOW_HEIGHT / map.height;
    let player = Player::new( 2.0, 7.0, 5, 1.0 );
    let (x, y) = player.get_world_pos( tile_width, tile_height );
    draw_rect( &mut framebuffer, x, y, player.size, player.size, 0x5555FFFF );

    // draw view cone and 3d view
    let fov = PI / 3.0;
    for i in 0..WINDOW_WIDTH / 2 {
        let angle = player.angle - fov / 2.0 + fov * i as f32 / WINDOW_WIDTH as f32;

        let mut ray_dist = 0.0;
        let draw_dist = 20.0;
        while ray_dist < draw_dist {
            let x = player.x + ray_dist * angle.cos();
            let y = player.y + ray_dist * angle.sin();
            if map.tiles[ x as usize + y as usize * map.width ] != Tile::Floor {
                let col_height = WINDOW_HEIGHT as f32 / (ray_dist * (angle - player.angle).cos());
                let x = WINDOW_WIDTH / 2 + i;
                let y = (WINDOW_HEIGHT as f32 - col_height) as usize / 2;
                draw_rect( &mut framebuffer, x, y, 1, col_height as usize, 0x000000FF );
                break;
            }

            let x = x * tile_width as f32;
            let y = y * tile_height as f32;
            draw_rect( &mut framebuffer, x as usize, y as usize, 1, 1, 0xFF000000 );
            ray_dist += 0.05;
        }
    }

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
    let tile_width = WINDOW_WIDTH / (map.width * 2);
    let tile_height = WINDOW_HEIGHT / map.height;

    for y in 0..map.height {
        for x in 0..map.width {
            match map.tiles[ x + y * map.width ] {
                Tile::Wall  => draw_rect(
                    buffer,
                    x * tile_width,
                    y * tile_height,
                    tile_width,
                    tile_height,
                    0x000000FF ),
                Tile::Floor => (),
            }
        }
    }
}

fn draw_rect( buffer: &mut [u32], x: usize, y: usize, w: usize, h: usize, color: u32 ) {
    for i in 0..w {
        for j in 0..h {
            let buf_x = x + i;
            let buf_y = y + j;
            buffer[ buf_x + buf_y * WINDOW_WIDTH ] = color;
        }
    }
}
