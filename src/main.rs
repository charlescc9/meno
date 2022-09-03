use bevy::prelude::*;

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32
}

fn create_particles(mut commands: Commands) {
    commands.spawn().insert(Particle).insert(Position {x: 1.0, y: 1.0});
    commands.spawn().insert(Particle).insert(Position {x: 2.0, y: 2.0});
}

fn print_position(query: Query<&Position, With<Particle>>) {
    for position in query.iter() {
        println!("This particle is at position: ({}, {})", position.x, position.y)
    }
}

fn main() {
    App::new()
        .add_startup_system(create_particles)
        .add_system(print_position)
        .run();
}
