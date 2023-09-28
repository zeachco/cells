use cell::draw_cells;
use cell::update_cells;
use constants::WORLD_UNITS;
use constants::{CELLS_PER_GENERATION, WORLD_PIXEL_SIZE};
use nannou::prelude::App;
use nannou::prelude::Frame;
use nannou::prelude::Update;
use nannou::prelude::*;
mod action;
mod brain;
mod cell;
mod constants;
mod tiles;
mod utilities;

use crate::cell::generate_cells;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WORLD_PIXEL_SIZE, WORLD_PIXEL_SIZE)
        .run();
}

struct Model {
    cells: Vec<cell::Cell>,
    tiles: tiles::Tiles,
}

fn model(_app: &App) -> Model {
    let tiles = tiles::Tiles::new(WORLD_UNITS + 1);
    let cells = generate_cells(CELLS_PER_GENERATION);
    Model { cells, tiles }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.tiles.update();
    update_cells(&mut model.cells, &mut model.tiles);
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    draw.xy(pt2(20.0, -30.0));
    model.tiles.draw(&draw);
    draw_cells(&draw, &model.cells);
    draw.to_frame(app, &frame).unwrap();
}
