mod hello;

pub struct HelloPlugin;

#[derive(Component)]
struct Person;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}

fn hello_world() {
    println!("hello world!");
}

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
