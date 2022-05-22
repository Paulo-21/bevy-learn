use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera3d;

use crate::{Player, TransparentCube};

#[derive(Default, Component)]
pub struct InputState {
    pitch: f32,
    yaw: f32,
}
#[derive(Clone, Copy, Component)]
pub enum AvailableMode {
    Fps,
    Top,
}
impl Default for AvailableMode {
    fn default() -> Self {
        AvailableMode::Fps
    }
}

#[derive(Component, Default)]
pub struct CamMode {
    pub mode : AvailableMode
}

pub fn camera_focus(
    mut query: ParamSet<(
        Query<&mut InputState, With<Camera3d>>,
        Query<&CamMode, With<Camera3d>>,
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Camera3d>>,
        Query<&mut Transform, With<TransparentCube>>,
    )>,
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    let window = windows.get_primary().unwrap();
    let mut yaw = 0.0;
    let mut pitch = 0.0;
    
    for mut info in query.p0().iter_mut() {
        for event in mouse_motion_events.iter() {
            info.pitch -= (0.00012 * event.delta.y * window.height()).to_radians();
            info.yaw -= (0.00012 * event.delta.x * window.width()).to_radians();
        }
        pitch = info.pitch;
        yaw = info.yaw;
    }
    let mut mode = AvailableMode::default();
    for q in query.p1().iter() {
        mode =  q.mode;
    }

    let player_query = query.p2();
    let player_transform = player_query.get_single().unwrap();
    let camera_pos: Vec3 = match mode {
        AvailableMode::Fps => {
            Vec3::new(player_transform.translation.x, player_transform.translation.y + 3.0, player_transform.translation.z)
        },
        AvailableMode::Top => {
            player_transform.translation + player_transform.rotation.mul_vec3(Vec3::new(0.0, 4.5, -6.0))
        }
    };
    let trans_cube_pos : Vec3 = player_transform.translation + player_transform.rotation.mul_vec3(Vec3::new(0.0, 1., 3.0));

    for mut transform in query.p3().iter_mut() {
        transform.translation = Vec3::new(camera_pos.x, camera_pos.y, camera_pos.z);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw)
        * Quat::from_axis_angle(Vec3::X, pitch);
    }
    for mut transform in query.p4().iter_mut() {
        transform.translation = Vec3::new(trans_cube_pos.x as i32 as f32, trans_cube_pos.y as i32 as f32, trans_cube_pos.z as i32 as f32);
    }
    
}
