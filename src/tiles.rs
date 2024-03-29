// 16 bits for a range from -32,768 to 32,767

use nannou::{prelude::rgb, Draw};
use rand::Rng;

use crate::{constants::UNIT_PIXEL_SIZE, utilities::coord_to_pixel};

pub struct Tile {
    x: i16,
    y: i16,
    pub properties: TileProperties,
}

pub struct TileProperties {
    pub energy_diffusion: f32,
    pub energy: f32,
}

impl Tile {
    pub fn new(x: i16, y: i16) -> Tile {
        return Tile {
            x,
            y,
            properties: TileProperties {
                energy_diffusion: rand::thread_rng().gen_range(0.01..0.1),
                energy: rand::thread_rng().gen_range(0.0..1.0),
            },
        };
    }

    pub fn change_energy(&mut self, offset: f32) {
        self.properties.energy += offset;
        if self.properties.energy < 0.0 {
            self.properties.energy = 0.0;
        }
        if self.properties.energy > 1.0 {
            self.properties.energy = 1.0;
        }
    }

    pub fn get_color(&self) -> (f32, f32, f32) {
        let r: f32 = self.properties.energy_diffusion;
        let g: f32 = self.properties.energy;
        let b: f32 = 0.0;
        return (r, g, b);
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

    pub fn get(&self, x: u16, y: u16) -> &Tile {
        if x > self.size || y > self.size {
            panic!("outofbounds")
        }

        return &self.tiles[x as usize][y as usize];
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Tile {
        return &mut self.tiles[x as usize][y as usize];
    }

    pub fn update(&mut self) {
        // for x in 0..self.size {
        //     for y in 0..self.size {
        //         let tile = self.get_mut(x, y);
        //         // tile.change_energy(-0.01);
        //     }
        // }
    }

    pub fn draw(&self, draw: &Draw) {
        let pad = 3.0;
        let dpad = 6.0;
        for x in 0..self.size {
            for y in 0..self.size {
                let tile = self.get(x, y);
                let (r, g, b) = tile.get_color();
                let (dx, dy) = coord_to_pixel(tile.x.into(), tile.y.into());
                draw.rect()
                    .x_y(dx + pad, dy + pad)
                    .w(UNIT_PIXEL_SIZE as f32 - dpad)
                    .h(UNIT_PIXEL_SIZE as f32 - dpad)
                    .color(rgb(r, g, b));
            }
        }
    }
}
