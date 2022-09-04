use bevy::prelude::*;

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

struct ParticleTimer(Timer);

fn create_particles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, 
                    mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn_bundle(PbrBundle { 
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(StandardMaterial { 
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
         }),
         ..default()
    });
    
    commands.spawn_bundle(PbrBundle { 
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.3, ..default() })),
        transform: Transform::from_translation(Vec3::new(1.0, 1.0, 0.5)),
        material: materials.add(Color::rgb(0.0, 0.0, 10.0).into()), ..default() 
    });

    commands.spawn_bundle(PbrBundle { 
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.1, ..default() })),
        transform: Transform::from_translation(Vec3::new(0.1, 0.1, 0.3)),
        material: materials.add(Color::rgb(10.0, 0.0, 0.0).into()), ..default() 
    });

    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02
    });

    commands.spawn_bundle(PointLightBundle { 
        point_light: PointLight { intensity: 1500.0, shadows_enabled: true, ..default() },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -10.0,
                right: 10.0,
                bottom: -10.0,
                top: 10.0,
                near: -100.0,
                far: 100.0,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn().insert(Particle).insert(Position { x: 1.0, y: 1.0 });
    commands.spawn().insert(Particle).insert(Position { x: 2.0, y: 2.0 });
}

fn print_position(time: Res<Time>, mut timer: ResMut<ParticleTimer>,
                  query: Query<&Position, With<Particle>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for position in query.iter() {
            println!("This particle is at position: ({}, {})", position.x, position.y)
        }
    }
}

struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParticleTimer(Timer::from_seconds(3.0, true)))
            .add_startup_system(create_particles)
            .add_system(print_position);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlePlugin)
        .run();
}
