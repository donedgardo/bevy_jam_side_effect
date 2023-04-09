use bevy::prelude::{Added, Commands, Component, default, Entity, GlobalTransform, Query, Res, ResMut, SpriteBundle, SpriteSheetBundle, TextureAtlas, Transform, Visibility, With, Without};
use bevy_ecs_ldtk::EntityInstance;
use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec2;
use bevy_rapier2d::dynamics::{GravityScale, RigidBody, Velocity};
use benimator::FrameRate;
use bevy::hierarchy::BuildChildren;
use crate::{InteractLightBeam, Ship};
use crate::animation::{Animation, AnimationState};
use crate::movement::Speed;

pub fn spawn_entity_instances(
    mut commands: Commands,
    player_q: Query<(Entity, &EntityInstance, &Transform, &GlobalTransform), (Added<EntityInstance>, Without<Ship>)>,
    mut bob_ship_q: Query<&mut Transform, With<Ship>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for
    (entity, instance, p_transform, global_transform)
    in player_q.iter() {
        match instance.identifier.as_ref() {
            "Player" => {
                if bob_ship_q.is_empty() {
                    println!("Creating Bob's Ship");
                    let texture_handle = asset_server.load("Bob's Ship-sheet.png");
                    let texture_atlas =
                        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 1, None, None);
                    let texture_atlas_handle = texture_atlases.add(texture_atlas);
                    let bob_bundle = (
                        Ship,
                        SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle,
                            transform: *p_transform,
                            ..default()
                        },
                        RigidBody::Dynamic,
                        GravityScale(0.),
                        Velocity::zero(),
                        Speed(90.),
                        AnimationState::default()
                    );
                    commands.entity(entity).insert(bob_bundle).with_children(|parent| {
                        let mut light_beam_translation = Transform::from(*global_transform);
                        light_beam_translation.translation.x += 8. * 9.;
                        light_beam_translation.translation.z += 1.;
                        parent.spawn((InteractLightBeam,
                                      SpriteBundle {
                                          texture: asset_server.load("Laser Lvl 1.png"),
                                          transform: light_beam_translation,
                                          visibility: Visibility::Hidden,
                                          ..default()
                                      }));
                    });
                } else {
                    println!("Moving Bob's Ship");
                    for mut transform in bob_ship_q.iter_mut() {
                        transform.translation = p_transform.translation;
                    }
                }
            }
            "LightSpeed" => {
                let texture_handle = asset_server.load("light speed.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle, Vec2::new(448., 224.), 16, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                let animation = Animation(benimator::Animation::from_indices(
                    0..=15,
                    FrameRate::from_fps(10.0),
                ));
                commands.entity(entity).insert((
                    LightSpeed,
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: *p_transform,
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    animation,
                    AnimationState::default(),
                ));
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct LightSpeed;
