use bevy::prelude::{Camera, CursorMoved, EventReader, GlobalTransform, Query, Transform, Window, With};
use bevy::math::{Quat, Vec2, Vec3};
use bevy::window::PrimaryWindow;
use crate::ship::Ship;

pub fn my_cursor_system(
    windows_query: Query<&Window, With<PrimaryWindow>>,
    cursor_evr: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut indicator_q: Query<(&mut Transform, &GlobalTransform), With<Ship>>,
) {
    if cursor_evr.len() == 0 || q_camera.is_empty() { return; }
    let (camera, camera_transform) = q_camera.single();
    let wnd = windows_query.single();
    if let Some(screen_pos) = wnd.cursor_position() {
        let cursor_pos = get_cursor_translation(camera, camera_transform, wnd, screen_pos);
        for (
            mut indicator_transform, cursor_global_transform
        ) in indicator_q.iter_mut() {
            let player_pos = cursor_global_transform.translation().truncate();
            let rotation = get_rotation_from_to(player_pos, cursor_pos);
            indicator_transform.rotation = rotation;
        }
    }
}

fn get_cursor_translation(camera: &Camera, camera_transform: &GlobalTransform, wnd: &Window, screen_pos: Vec2) -> Vec2 {
    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    world_pos.truncate()
}

fn get_rotation_from_to(from: Vec2, to: Vec2) -> Quat {
    let diff = to - from;
    //TODO: May not be deterministic across computers
    let angle = diff.y.atan2(diff.x);
    Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle)
}
