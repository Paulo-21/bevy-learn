use bevy::{prelude::*};
use bevy::window;
use bevy::window::WindowMode;
#[derive(Component)]
struct Player;

fn move_player(
    keys: Res<Input<KeyCode>>, 
    mut player_query: Query<&mut Transform, With<Player>>, 
) {
    let mut angle = 0.0;
    let mut direction = Vec3::ZERO;
    if keys.any_pressed([KeyCode::Z]) { direction.z += 1.; }
    if keys.any_pressed([KeyCode::S]) { direction.z -= 1.; }
    if keys.any_pressed([KeyCode::D]) { direction.x -= 1.; }
    if keys.any_pressed([KeyCode::Q]) { direction.x += 1.; }
    if keys.any_pressed([KeyCode::Up]) { direction.y += 1.; }
    if keys.any_pressed([KeyCode::Down]) { direction.y -= 1.; }
    if keys.any_pressed([KeyCode::Left]) { angle = 0.1; }
    if keys.any_pressed([KeyCode::Right]) { angle = -0.1; }
    if direction == Vec3::ZERO && angle == 0.0 { return; }

    let move_speed:f32 = 0.05;
    let move_delta = direction * move_speed;
    
    for mut transform in player_query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(angle));
        
        let r = transform.rotation.mul_vec3(move_delta);
        transform.translation  += r;
    }
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
) {
   commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn_bundle(TransformBundle::from(Transform {
        translation: Vec3::new(0.0,0.,0.),
        rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4),
        ..default()
    }))
    .with_children(|cell| {
        cell.spawn_scene(asset_server.load::<Scene, _>("models/gltf/character_rogue.gltf#Scene0"));
    }).insert(Player);

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            color: Color::YELLOW,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, 10.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
fn main() {
    println!("Bienvenue sur mon jeu");
    
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
    .insert_resource(WindowDescriptor {
        title: "Jeux video".to_string(),
        width : 500.0,
        height : 400.0,
        present_mode : window::PresentMode::Fifo,
        mode : WindowMode::Windowed,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(move_player)
    .run();
}