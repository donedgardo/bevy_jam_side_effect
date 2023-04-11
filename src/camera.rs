use bevy::prelude::{Added, Camera2dBundle, Changed, Color, Commands, Component, Query, Transform, With, Without};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use crate::ship::Ship;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_main_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::hex("#000").unwrap());
    camera_bundle.projection.scale *= 0.55;
    commands.spawn((MainCamera, camera_bundle));
}

pub fn camera_follow_ship(
    ship_q: Query<&Transform, (With<Ship>, Changed<Transform>, Without<MainCamera>)>,
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
) {
    for ship_transform in ship_q.iter() {
        camera_move_to(&mut camera_q, ship_transform);
    }
}

pub fn position_camera_at_ship(
    added_ship_q: Query<&Transform, (Added<Ship>, Without<MainCamera>)>,
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
) {
    for ship_transform in added_ship_q.iter() {
        let transform = Transform::from_xyz(
            ship_transform.translation.x,
            ship_transform.translation.y,
            999.,
        );
        camera_move_to(&mut camera_q, &transform);
    }
}

fn camera_move_to(camera_q: &mut Query<&mut Transform, With<MainCamera>>, transform: &Transform) {
    for mut camera_transform in camera_q.iter_mut() {
        camera_transform.translation.x = transform.translation.x;
        camera_transform.translation.y = transform.translation.y;
    }
}
