// 16 bits for a range from -32,768 to 32,767

use nannou::{prelude::BLACK, Draw};

use crate::{
    constants::{MAX_WORLD_COORD, MIN_WORLD_COORD, UNIT_PIXEL_SIZE},
    utilities::coord_to_pixel,
};

pub struct Tile {
    x: i16,
    y: i16,
    perperties: TileProperties,
}

struct TileProperties {
    energy_diffusion: f32,
    energy: f32,
}

impl Tile {
    pub fn new(x: i16, y: i16) -> Tile {
        return Tile {
            x,
            y,
            perperties: TileProperties {
                energy_diffusion: 0.1,
                energy: 0.5,
            },
        };
    }
}

pub struct Tiles {
    tiles: Vec<Vec<Tile>>,
    size: u16,
}

impl Tiles {
    pub fn new(size: u16) -> Tiles {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();

        for x in 0..size {
            let mut tiles_row: Vec<Tile> = Vec::new();
            for y in 0..size {
                let tile = Tile::new(x as i16, y as i16);
                tiles_row.push(tile);
            }
            tiles.push(tiles_row);
        }

        return Tiles { tiles, size };
    }

    pub fn get(&self, x: i16, y: i16) -> &Tile {
        return &self.tiles[x as usize][y as usize];
    }

    pub fn get_mut(&mut self, x: i16, y: i16) -> &mut Tile {
        return &mut self.tiles[x as usize][y as usize];
    }

    pub fn draw(&self, draw: &Draw) {
        let mut x = MIN_WORLD_COORD + 1;
        let mut y = MIN_WORLD_COORD + 1;
        while x < MAX_WORLD_COORD {
            while y < MAX_WORLD_COORD {
                let (dx, dy) = coord_to_pixel(x, y);
                draw.rect()
                    .x_y(dx, dy)
                    .w_h(UNIT_PIXEL_SIZE as f32, UNIT_PIXEL_SIZE as f32)
                    .color(BLACK);
                y += 1;
            }
            x += 1;
            y = MIN_WORLD_COORD + 1;
        }
    }
}
