use nannou::color::named::PURPLE;
use nannou::prelude::App;
use nannou::prelude::Frame;
use nannou::prelude::Update;
mod world;
use world::WorldObject;

fn main() {
    let mut world_matrix: Vec<Vec<WorldObject>> = Vec::new();

    // for _i in 0..height {
    //     let mut row: Vec<WorldObject> = Vec::new();
    //     for _j in 0..width {
    //         let cells = vec![/* some Cell instances */];
    //         let properties = WorldProperties { temperature: 0.0, humidity: 0.0 };
    //         let world_object = WorldObject { cells, properties };
    //         row.push(world_object);
    //     }
    //     world_matrix.push(row);
    // }

    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(PURPLE);
}
