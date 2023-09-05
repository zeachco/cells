use nannou::Draw;
use rand::Rng;

use crate::{
    brain::take_decision,
    constants::{
        CELL_MAX_ENERGY, MAX_WORLD_COORD, MIN_WORLD_COORD, OUT_OF_BOUND_ENERGY_LOSS,
        UNIT_PIXEL_SIZE, WORLD_UNITS,
    },
    utilities::coord_to_pixel,
};

pub struct Cell {
    pub energy: f32,
    pub adn: Option<String>,
    pub x: i32,
    pub y: i32,
}

pub fn generate_cells(count: u32) -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![];
    for _ in 0..count {
        let mut rng = rand::thread_rng();
        let from: i32 = WORLD_UNITS as i32 / -2;
        let to: i32 = WORLD_UNITS as i32 / 2;
        let x = rng.gen_range(from..to);
        let y = rng.gen_range(from..to);
        cells.push(Cell {
            energy: 1000.0,
            x,
            y,
            adn: None,
        });
    }
    cells
}

pub fn update_cells(cells: &mut Vec<Cell>) {
    for cell in cells {
        take_decision(cell);
        handle_out_of_bound(cell);
        limit_energy(cell);
    }
}

pub fn draw_cells(draw: &Draw, cells: &Vec<Cell>) {
    for cell in cells {
        let (x, y) = coord_to_pixel(cell.x, cell.y);
        draw.rect()
            .color(nannou::color::named::STEELBLUE)
            .w(UNIT_PIXEL_SIZE.into())
            .h(UNIT_PIXEL_SIZE.into())
            .x_y(x, y);
    }
}

fn handle_out_of_bound(cell: &mut Cell) {
    if cell.x > MAX_WORLD_COORD {
        cell.x = MAX_WORLD_COORD;
        cell.energy -= OUT_OF_BOUND_ENERGY_LOSS;
    }
    if cell.x < MIN_WORLD_COORD {
        cell.x = MIN_WORLD_COORD;
        cell.energy -= OUT_OF_BOUND_ENERGY_LOSS;
    }
    if cell.y > MAX_WORLD_COORD {
        cell.y = MAX_WORLD_COORD;
        cell.energy -= OUT_OF_BOUND_ENERGY_LOSS;
    }

    if cell.y < MIN_WORLD_COORD {
        cell.y = MIN_WORLD_COORD;
        cell.energy -= OUT_OF_BOUND_ENERGY_LOSS;
    }
}

fn limit_energy(cell: &mut Cell) {
    if cell.energy > CELL_MAX_ENERGY {
        cell.energy = CELL_MAX_ENERGY;
    }
}
