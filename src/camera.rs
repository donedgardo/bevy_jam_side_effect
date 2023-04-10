use bevy::prelude::{Camera2dBundle, Color, Commands, Component};
use bevy::core_pipeline::clear_color::ClearColorConfig;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_main_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::hex("#000").unwrap());
    camera_bundle.projection.scale *= 0.55;
    commands.spawn((MainCamera, camera_bundle));
}
