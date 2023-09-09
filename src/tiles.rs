// 16 bits for a range from -32,768 to 32,767

use nannou::{color::ConvertInto, prelude::rgb, text::Wrap, Draw};
use rand::Rng;

use crate::{constants::UNIT_PIXEL_SIZE, utilities::coord_to_pixel};

pub struct Tile {
    x: i16,
    y: i16,
    properties: TileProperties,
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
            properties: TileProperties {
                energy_diffusion: rand::thread_rng().gen_range(0.0..1.0),
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
        if x < 0 || x > self.size || y < 0 || y > self.size {
            panic!("outofbounds")
        }

        return &self.tiles[x as usize][y as usize];
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Tile {
        return &mut self.tiles[x as usize][y as usize];
    }

    pub fn update(&mut self) {
        for x in 0..self.size {
            for y in 0..self.size {
                let mut tile = self.get_mut(x, y);
                tile.change_energy(-0.01);
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        for x in 0..self.size {
            for y in 0..self.size {
                let tile = self.get(x, y);
                let r: f32 = tile.properties.energy;
                let g: f32 = tile.properties.energy;
                let b: f32 = tile.properties.energy_diffusion * 0.1;
                let (dx, dy) = coord_to_pixel(tile.x.into(), tile.y.into());
                draw.rect()
                    .x_y(dx + 1.0, dy + 1.0)
                    .w(UNIT_PIXEL_SIZE as f32 - 2.0)
                    .h(UNIT_PIXEL_SIZE as f32 - 2.0)
                    .color(rgb(r, g, b));
            }
        }
    }
}
