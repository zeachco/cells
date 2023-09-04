use nannou::Draw;
use rand::Rng;

use crate::{
    constants::{WORLD_CELL_SIZE, WORLD_SIZE},
    utilities::coord_to_pixel,
};

pub struct Cell {
    pub adn: Option<String>,
    pub x: i16,
    pub y: i16,
}

pub fn generate_cells(count: u32) -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![];
    for i in 0..count {
        let mut rng = rand::thread_rng();
        let x: i16 = rng.gen_range(-WORLD_SIZE / 2..WORLD_SIZE / 2);
        let y: i16 = rng.gen_range(-WORLD_SIZE / 2..WORLD_SIZE / 2);
        cells.push(Cell { x, y, adn: None });
    }
    cells
}

pub fn draw_cells(draw: &Draw, cells: &Vec<Cell>) {
    for cell in cells {
        let (x, y) = coord_to_pixel(cell.x, cell.y);
        draw.rect()
            .color(nannou::color::named::STEELBLUE)
            .w(WORLD_CELL_SIZE as f32)
            .h(WORLD_CELL_SIZE as f32)
            .x_y(x.into(), y.into());
    }
}
