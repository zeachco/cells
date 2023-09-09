use cell::draw_cells;
use cell::update_cells;
use constants::WORLD_PIXEL_SIZE;
use nannou::color::named::LIGHTGRAY;
use nannou::prelude::App;
use nannou::prelude::Frame;
use nannou::prelude::Update;
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
    let cells = generate_cells(100);
    Model {
        cells,
        tiles: tiles::Tiles::new(50),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.tiles.update();
    update_cells(&mut model.cells);
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(LIGHTGRAY);
    let draw = app.draw();
    model.tiles.draw(&draw);
    draw_cells(&draw, &model.cells);
    draw.to_frame(app, &frame).unwrap();
}
