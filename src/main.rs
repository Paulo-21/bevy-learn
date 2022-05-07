use bevy::{prelude::*};
use bevy::window;
use bevy::window::WindowMode;
use std::time::{Instant};

#[derive(Component)]
struct Player;
#[derive(Component)]
struct TimeStampUpdate(Instant);

fn spawn_camera (mut commands : Commands) {
   let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);
}

fn move_player(
    keys: Res<Input<KeyCode>>, 
    mut player_query: Query<&mut Transform, With<Player>>, 
) {
    let mut direction = Vec3::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if direction == Vec3::ZERO {
        return;
    }
    let move_speed:f32 = 0.05;
    let move_delta = (direction * move_speed);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0.47, 1.),
            custom_size: Some(Vec2::new(1., 1.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player);
}
fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
) {
   commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Player);
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 0.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
fn main() {
    println!("Bienvenue sur mon jeu");
    
        App::new()
        // Set antialiasing to use 4 samples
        //.insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .insert_resource(WindowDescriptor {
            title: "Jeux video".to_string(),
            present_mode : window::PresentMode::Fifo,
            mode : WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        //.add_startup_system(spawn_camera)
        //.add_startup_system(spawn_player)
        .add_startup_system(setup)
        .add_system(move_player)

        .run();
}