use bevy::prelude::*;
use hello;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(hello::HelloPlugin)
        .run();
}
