use cell::draw_cells;
use nannou::color::named::LIGHTGRAY;
use nannou::prelude::App;
use nannou::prelude::Frame;
use nannou::prelude::Update;
mod cell;
mod constants;
mod utilities;
mod world;
use constants::WORLD_CELL_SIZE;
use constants::WORLD_SIZE;
use world::generate_world;

use crate::cell::generate_cells;

fn main() {
    generate_world();
    let size = (WORLD_SIZE * WORLD_CELL_SIZE) as u32;
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(size, size)
        .run();
}

struct Model {
    cells: Vec<cell::Cell>,
}

fn model(app: &App) -> Model {
    let cells = generate_cells(50);
    Model { cells }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(LIGHTGRAY);
    let draw = app.draw();
    draw_cells(&draw, &model.cells);
    draw.to_frame(app, &frame).unwrap();
}
