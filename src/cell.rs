use nannou::Draw;
use rand::Rng;

use crate::{
    action::CellAction,
    brain::{take_decision, Brain},
    constants::{
        CELL_MAX_ENERGY, CELL_MIN_ENERGY_TO_FUNCTION, MAX_WORLD_COORD, MIN_WORLD_COORD,
        OUT_OF_BOUND_ENERGY_LOSS, UNIT_PIXEL_SIZE, WORLD_UNITS,
    },
    tiles::Tiles,
    utilities::coord_to_pixel,
};

pub struct Cell {
    pub energy: f32,
    pub adn: Option<String>,
    pub x: i32,
    pub y: i32,
    pub brain: Brain,
    pub color: (f32, f32, f32, f32),
}

fn get_cell_color() -> (f32, f32, f32, f32) {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen();
    let g: f32 = rng.gen();
    let b: f32 = rng.gen();
    (r, g, b, 0.5)
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        return Cell {
            energy: 1000.0,
            adn: None,
            x,
            y,
            brain: Brain::new(),
            color: get_cell_color(),
        };
    }

    pub fn act(&mut self, action: CellAction) {
        if self.energy < CELL_MIN_ENERGY_TO_FUNCTION {
            return;
        }
        match action {
            CellAction::Up => {
                self.energy -= 0.1;
                self.y += 1;
            }
            CellAction::Down => {
                self.energy -= 0.1;
                self.y -= 1;
            }
            CellAction::Right => {
                self.energy -= 0.1;
                self.x += 1;
            }
            CellAction::Left => {
                self.energy -= 0.1;
                self.x -= 1;
            }
            CellAction::Give => {
                self.energy -= 1.0;
            }
            CellAction::Take => {
                self.energy += 1.0;
            }
            CellAction::Chill => {
                self.energy -= 0.01;
            }
        }
    }
}

pub fn generate_cells(count: usize) -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![];
    for _ in 0..count {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..WORLD_UNITS as i32);
        let y = rng.gen_range(0..WORLD_UNITS as i32);
        cells.push(Cell::new(x, y));
    }
    cells
}

pub fn update_cells(cells: &mut Vec<Cell>, tiles: &mut Tiles) {
    for cell in cells {
        take_decision(cell, tiles);
        handle_out_of_bound(cell);
        limit_energy(cell);
    }
}

pub fn draw_cells(draw: &Draw, cells: &Vec<Cell>) {
    for cell in cells {
        let (x, y) = coord_to_pixel(cell.x, cell.y);
        let (r, g, b, a) = cell.color;
        draw.rect()
            .color(nannou::color::rgba(r, g, b, a))
            .w_h(UNIT_PIXEL_SIZE.into(), UNIT_PIXEL_SIZE.into())
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
