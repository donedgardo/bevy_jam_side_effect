use bevy::prelude::{Commands, Component, Entity, EventReader, Query, Transform, With};
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy::hierarchy::Parent;
use bevy_rapier2d::dynamics::Velocity;
use bevy::math::Vec2;
use crate::level::AggroRange;
use crate::movement::Speed;
use crate::ship::Ship;

#[derive(Component)]
pub struct Aggro(pub Entity);

pub fn handle_aggro(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    ship_q: Query<Entity, With<Ship>>,
    aggro_range_q: Query<&Parent, With<AggroRange>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok(ship) = ship_q.get(*e1) {
                    if let Ok(parent) = aggro_range_q.get(*e2) {
                        commands.entity(parent.get()).insert(Aggro(ship));
                    }
                } else if let Ok(ship) = ship_q.get(*e2) {
                    if let Ok(parent) = aggro_range_q.get(*e1) {
                        commands.entity(parent.get()).insert(Aggro(ship));
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if ship_q.get(*e1).is_ok() {
                    if let Ok(parent) = aggro_range_q.get(*e2) {
                        commands.entity(parent.get()).remove::<Aggro>().insert(Velocity::zero());
                    }
                } else if ship_q.get(*e2).is_ok() {
                    if let Ok(parent) = aggro_range_q.get(*e1) {
                        commands.entity(parent.get()).remove::<Aggro>().insert(Velocity::zero());
                    }
                }
            }
        }
    }
}

pub fn aggro_movement(
    mut aggro_query: Query<(&Aggro, &Speed, &Transform, &mut Velocity)>,
    transform_q: Query<&Transform>,
) {
    for (aggro, speed, aggro_transform, mut velocity) in aggro_query.iter_mut() {
        if let Ok(transform) = transform_q.get(aggro.0) {
            let difference = transform.translation - aggro_transform.translation;
            velocity.linvel = Vec2::new(difference.x, difference.y).normalize_or_zero() * speed.0;
        }
    }
}
