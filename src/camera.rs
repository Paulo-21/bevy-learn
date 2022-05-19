use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera3d;

use crate::Player;

#[derive(Default)]
pub struct InputState {
    pitch: f32,
    yaw: f32,
}
#[allow(dead_code)]
pub enum CameraMode {
    Fps,
    Top,
}
impl Default for CameraMode {
    fn default() -> Self {
        CameraMode::Fps
    }
}
#[derive(Component)]
pub struct CameraInfo {
    pub input_state: InputState,
    pub camera_mode: CameraMode,
}

pub fn camera_focus(
    mut query: ParamSet<(
        Query<&mut CameraInfo, With<Camera3d>>,
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Camera3d>>,
    )>,
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    let window = windows.get_primary().unwrap();
    let mut yaw = 0.0;
    let mut pitch = 0.0;
    for mut info in query.p0().iter_mut() {
        for event in mouse_motion_events.iter() {
            info.input_state.pitch -= (0.00012 * event.delta.y * window.height()).to_radians();
            info.input_state.yaw -= (0.00012 * event.delta.x * window.width()).to_radians();
        }
        pitch = info.input_state.pitch;
        yaw = info.input_state.yaw;
    }
    let camera_pos = {
        let player_query = query.p1();
        let player_transform = player_query.get_single().unwrap();
        player_transform.translation + player_transform.rotation.mul_vec3(Vec3::new(0.0, 4.5, -6.0))
    };

    for mut transform in query.p2().iter_mut() {
        transform.translation = Vec3::new(camera_pos.x, camera_pos.y, camera_pos.z);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw)
        * Quat::from_axis_angle(Vec3::X, pitch);
    }
    
}
