use bevy::prelude::{Commands, Entity, EventReader, NextState, Query, Res, ResMut, Time, Timer, TimerMode, Visibility, With};
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy::hierarchy::Children;
use std::time::Duration;
use crate::AppState;
use crate::beams::{InteractLightBeam, UnderBeamItems};
use crate::level::{Damage, DamageCollider, Health, Item, ResourceNameplate};
use crate::ship::Ship;

pub fn handle_damage(
    time: Res<Time>,
    mut damage_q: Query<(&mut Damage, &mut Health), With<Ship>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (mut damage, mut health) in damage_q.iter_mut() {
        damage.1.tick(time.delta());
        if damage.1.just_finished() {
            health.current -= damage.0;
        }
        if health.current <= 0. {
            next_state.set(AppState::GameOver);
        }
    }
}

pub fn handle_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    herbs_query: Query<(&Children, &Item)>,
    resource_label_query: Query<&ResourceNameplate>,
    beam_q: Query<&InteractLightBeam>,
    mut under_beam: ResMut<UnderBeamItems>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok((children, _)) = herbs_query.get(*e1) {
                    if beam_q.get(*e2).is_ok() {
                        under_beam.0.push(*e1);
                        for child in children.iter() {
                            if resource_label_query.get(*child).is_ok() {
                                commands.entity(*child).insert(Visibility::Visible);
                            }
                        }
                    }
                } else if let Ok((children, _)) = herbs_query.get(*e2) {
                    if beam_q.get(*e1).is_ok() {
                        under_beam.0.push(*e2);
                        for child in children.iter() {
                            if resource_label_query.get(*child).is_ok() {
                                commands.entity(*child).insert(Visibility::Visible);
                            }
                        }
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                for entity in vec![e1, e2].into_iter() {
                    if let Ok((children, _)) = herbs_query.get(*entity) {
                        if let Some(index) = under_beam.0.iter().position(|x| x == entity) {
                            under_beam.0.remove(index);
                        };
                        for child in children.iter() {
                            if resource_label_query.get(*child).is_ok() {
                                commands.entity(*child).insert(Visibility::Hidden);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_collision_damage(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    ship_q: Query<Entity, With<Ship>>,
    damage_q: Query<&DamageCollider>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok(damage) = damage_q.get(*e1) {
                    if let Ok(ship) = ship_q.get(*e2) {
                        commands.entity(ship).insert(Damage(damage.0, Timer::new(Duration::from_millis(150), TimerMode::Repeating)));
                    }
                } else if let Ok(damage) = damage_q.get(*e2) {
                    if let Ok(ship) = ship_q.get(*e1) {
                        commands.entity(ship).insert(Damage(damage.0, Timer::new(Duration::from_millis(150), TimerMode::Repeating)));
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if damage_q.get(*e1).is_ok() {
                    if let Ok(ship) = ship_q.get(*e2) {
                        commands.entity(ship).remove::<Damage>();
                    }
                } else if damage_q.get(*e2).is_ok() {
                    if let Ok(ship) = ship_q.get(*e1) {
                        commands.entity(ship).remove::<Damage>();
                    }
                }
            }
        }
    }
}
