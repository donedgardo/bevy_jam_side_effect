use bevy::prelude::{Commands, Component, Entity, EventWriter, KeyCode, MouseButton, Query, Res, ResMut, Resource, Visibility, With};
use bevy::input::Input;
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Sensor};
use bevy::math::Vec2;
use crate::animation;
use crate::animation::Animation;
use crate::level::{Inventory, Item};
use crate::movement::Speed;
use crate::ship::Ship;

#[derive(Component)]
pub struct InteractLightBeam;

#[derive(Resource)]
pub struct UnderBeamItems(pub Vec<Entity>);

pub struct BeamUpEvent(pub Entity);

pub fn beam_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    beam_q: Query<Entity, With<InteractLightBeam>>,
    ship_q: Query<Entity, With<Ship>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for beam in beam_q.iter() {
            commands.entity(beam).insert(
                (Visibility::Visible,
                 Collider::triangle(Vec2::new(-55., 0.),
                                    Vec2::new(56., 18.),
                                    Vec2::new(56., -18.)),
                 ActiveEvents::COLLISION_EVENTS,
                 Sensor));
        }
        for ship in ship_q.iter() {
            animation::add_blinking_animation(&mut commands, ship);
        }
    }
    if mouse_input.just_released(MouseButton::Left) {
        for beam in beam_q.iter() {
            commands.entity(beam).insert(Visibility::Hidden);
            commands.entity(beam).remove::<Collider>().remove::<Sensor>();
        }
        for ship in ship_q.iter() {
            commands.entity(ship).remove::<Animation>();
        }
    }
}

#[cfg(test)]
mod beam_tests {
    use bevy::input::{ButtonState, InputPlugin};
    use bevy::input::keyboard::KeyboardInput;
    use bevy::prelude::*;
    use super::*;

    #[test]
    fn it_adds_to_inventory_when_beam_up() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugin(InputPlugin::default());
        app.add_event::<BeamUpEvent>();
        app.insert_resource(UnderBeamItems(vec![Entity::from_raw(0)]));
        app.add_system(beam_up);

        let player = app.world.spawn(
            (Ship,Inventory::new(30))
        ).id();

        app.world.send_event(KeyboardInput {
            scan_code: 0,
            key_code: Option::from(KeyCode::Space),
            state: ButtonState::Pressed,
        });

        app.update();

        let inventory = app.world.entity(player).get::<Inventory>();
        assert_eq!(inventory.iter().len(), 1);
    }
}


pub fn beam_up(
    input: Res<Input<KeyCode>>,
    item_query: Query<&Item>,
    mut ev_beam_up: EventWriter<BeamUpEvent>,
    mut inventory_query: Query<&mut Inventory, With<Ship>>,
    mut under_beam: ResMut<UnderBeamItems>,
) {
    if !input.just_pressed(KeyCode::Space) || under_beam.0.is_empty() { return; }
    let beamed_entity = under_beam.0.pop().unwrap();
    if let Ok(item) = item_query.get(beamed_entity) {
        if let Ok(mut inventory) = inventory_query.get_single_mut() {
            inventory.add(item);
            ev_beam_up.send(BeamUpEvent(beamed_entity));
        }
    }
}

pub fn boost_input(
    mut commands: Commands,
    mut ship_q: Query<(Entity, &mut Speed), With<Ship>>,
    key_input: Res<Input<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::LShift) {
        for (ship, mut speed) in ship_q.iter_mut() {
            speed.0 += 70.;
            animation::add_blinking_animation(&mut commands, ship);
        }
    }
    if key_input.just_released(KeyCode::LShift) {
        for (ship, mut speed) in ship_q.iter_mut() {
            speed.0 -= 70.;
            commands.entity(ship).remove::<Animation>();
        }
    }
}



