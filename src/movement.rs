use bevy::prelude::{Component, KeyCode, Query, Res, With};
use bevy::input::Input;
use bevy::math::Vec2;
use bevy_rapier2d::dynamics::Velocity;
use crate::ship::Ship;

pub fn movement_input(
    mut player_q: Query<(&mut Velocity, &Speed), With<Ship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut velocity, speed) in player_q.iter_mut() {
        let mut direction = Vec2::default();
        handle_keyboard_input(&keyboard_input, &mut direction);
        velocity.linvel = direction.normalize_or_zero() * speed.0;
    };
}

fn handle_keyboard_input(keyboard_input: &Res<Input<KeyCode>>, direction: &mut Vec2) {
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
}

#[derive(Component)]
pub struct Speed(pub f32);
