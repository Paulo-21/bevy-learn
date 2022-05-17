use bevy::{prelude::*};
use bevy::window;
use bevy::window::WindowMode;
use bevy::render::camera::Camera3d;
use bevy_rapier3d::prelude::*;
use rstar::{RTree, RTreeObject, AABB, PointDistance};

#[derive(Component, Default)]
struct Player;
#[derive(Component)]
#[allow(dead_code)]
struct Block {
    entity : Option<Entity>,
    size : f32,
    x : f32,
    y : f32,
    z : f32,
}
#[derive(Default)]
struct Game {
    map : RTree<Block>,
    player : Option<Entity>,
}
impl RTreeObject for Block {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x, self.y, self.z])
    }
}
impl PointDistance for Block
{
    fn distance_2(&self, point: &[f32; 3]) -> f32
    {
        let d_x = self.x - point[0];
        let d_y = self.y - point[1];
        let d_z = self.z - point[2];
        ((d_x * d_x).powi(2) + (d_y * d_y).powi(2) + (d_z * d_z).powi(2)).sqrt()
    }
}

fn intersect (a : Vec3, b : &Block) -> bool{
    ((a.x-b.x).powi(2) + (a.y -b.y).powi(2) + (a.z-b.z).powi(2)).sqrt() < 1.25
    /*return (a.x < b.x + b.size && a.x + b.size > b.x) &&
         (a.y < b.y + b.size && a.y + b.size > b.y) &&
         (a.z < b.z + b.size && a.z + b.size > b.z);*/
}

fn move_player(
    keys: Res<Input<KeyCode>>, 
    mut player_query: Query<&mut Transform, With<Player>>,
    mut player_query_velocity: Query<&mut Velocity, With<Player>>,
    mut commands : Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
    mut game : ResMut<Game>,
    time : Res<Time>,
) {
    let mut player_transform = player_query.single_mut();
    let mut player_velocity = player_query_velocity.single_mut();
    
    let mut angley = 0.0;
    let mut anglex = 0.0;
    let mut direction = Vec3::ZERO;
    if keys.any_pressed([KeyCode::Z]) { direction.z += 1.; }
    if keys.any_pressed([KeyCode::S]) { direction.z -= 1.; }
    if keys.any_pressed([KeyCode::D]) { direction.x -= 1.; }
    if keys.any_pressed([KeyCode::Q]) { direction.x += 1.; }
    /*if keys.any_pressed([KeyCode::Up]) { direction.y += 1.; }
    if keys.any_pressed([KeyCode::Down]) { direction.y -= 1.; }*/
    if keys.any_pressed([KeyCode::Left]) { angley = 0.05; }
    if keys.any_pressed([KeyCode::Right]) { angley = -0.05; }
    if keys.any_pressed([KeyCode::Up]) { anglex = 0.05; }
    if keys.any_pressed([KeyCode::Down]) { anglex = -0.05; }
    if keys.any_just_pressed([KeyCode::Space]) {
        if player_velocity.linvel.y < 0.1 {
            player_velocity.linvel.y = 10.0;
        }
    }
    //println!("{}", player_velocity.linvel.y);
    if keys.any_just_pressed([KeyCode::E]) {
        let v = Vec3::from([0.0, 5.5, 2.0]);
        let f = player_transform.rotation.mul_vec3(v);
        let handler = commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size : 1.0,
            })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(
                ((player_transform.translation.x + f.x) as i32) as f32,
                ((player_transform.translation.y + f.y) as i32) as f32,
                ((player_transform.translation.z + f.z) as i32) as f32 
            ) ,
                ..default()
        })
        .insert(RigidBody::Dynamic)
        //.insert(Transform::from_xyz(0.0, 5.0, 0.0))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(GravityScale(1.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .id();
        game.map.insert( Block {
            entity : Some(handler),
            size : 1.0,
            x : player_transform.translation.x + f.x,
            y : player_transform.translation.y + f.y,
            z : player_transform.translation.z + f.z,
        });
    }
    if direction == Vec3::ZERO && angley == 0.0 && anglex == 0.0{ return; }

    let move_speed:f32 = 5.0;
    let move_delta = direction * ((move_speed * time.delta().as_millis() as f32) / 1000.0);

    player_transform.rotate(Quat::from_rotation_y(angley));
    player_transform.rotate(Quat::from_rotation_x(anglex));
    let r = player_transform.rotation.mul_vec3(move_delta);
    /* 
    let pos = player_transform.translation;
    if let Some(block) = game.map.nearest_neighbor(&[pos.x, pos.y, pos.z]) {
        if !intersect(player_transform.translation+r, block) {
            player_transform.translation  += r;
        }
    }
    else {
        player_transform.translation  += r;
    }
    */
    player_transform.translation  += r;
}
fn camera_focus(
    game: ResMut<Game>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
    let (trans, rot) = if let Some(player_entity) = game.player {
        let vec = if let Ok(player_transform) = transforms.p1().get(player_entity) {
            (player_transform.translation, player_transform.rotation)
        } else {
            (Vec3::ZERO, Quat::from_array([0.0,0.0,0.0, 0.0]))
        };
        vec
    } else {
        (Vec3::ZERO, Quat::from_array([0.0,0.0,0.0, 0.0]))
    };
    for mut transform in transforms.p0().iter_mut() {
        let camera_pos = trans + rot.mul_vec3(Vec3::new(0.0, 4.5, -6.0));
        *transform = Transform::from_xyz(camera_pos.x, camera_pos.y, camera_pos.z).looking_at(trans, Vec3::Y);
    }

}
fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game : ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
) {
   commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
        material: materials.add(Color::GRAY.into()),
        //material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Collider::cuboid(100.0, 0.1, 100.0));
    game.player = Some(commands.spawn_bundle(TransformBundle::from(Transform {
        translation: Vec3::new(0.0,0.,0.),
        rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4),
        ..default()
    }))
    .with_children(|cell| {
        cell.spawn_scene(asset_server.load::<Scene, _>("models/gltf/character_mage.gltf#Scene0"));
    })
    .insert(Player)
    .insert(RigidBody::Dynamic)
    //.insert(Transform::from_xyz(0.0, 5.0, 0.0))
    .insert(Velocity {
        linvel: Vec3::new(0.0, 0.0, 0.0),
        angvel: Vec3::new(0.0, 0.0, 0.0),
    })
    .insert(Collider::cuboid(0.5, 0.5, 0.5))
    .insert(GravityScale(1.5))
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(LockedAxes::ROTATION_LOCKED)
    .id());

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 6.0, 10.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 4.5, -6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    println!("Bienvenue sur mon jeu");
    
    App::new()
    .init_resource::<Game>()
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
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(setup)
    .add_system(move_player)
    .add_system(camera_focus)
    .run();
}