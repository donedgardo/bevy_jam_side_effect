use bevy::prelude::*;
use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec2;
use bevy::hierarchy::BuildChildren;
use bevy_ecs_ldtk::EntityInstance;
use bevy_rapier2d::dynamics::{GravityScale, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, Sensor};
use benimator::FrameRate;

use crate::{InteractLightBeam, Ship};
use crate::animation::{Animation, AnimationState};
use crate::movement::Speed;

#[derive(Component)]
pub struct Herbs {
    pub(crate) description: String,
    texture: Handle<Image>,
}

#[derive(Component)]
pub struct ResourceNameplate;

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
                        Collider::triangle(Vec2::new(-11., -16.), Vec2::new(-11., 16.), Vec2::new(16., 0.)),
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
            "Herbs" => {
                let texture_handle = asset_server.load("resources.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16., 16.), 1, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands.entity(entity).insert((
                    Herbs {
                        description: "Herbs:\nGain Life Support.\nSide Effects: ???.\nRight-click: Beam up.".to_string(),
                        texture: texture_handle.clone(),
                    },
                    Collider::ball(8.),
                    Sensor,
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: *p_transform,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        ResourceNameplate,
                        Text2dBundle {
                            text: Text::from_section("Herbs", TextStyle {
                                font: asset_server.load("fonts/static/JetBrainsMono-Light.ttf"),
                                font_size: 16.,
                                ..default()
                            }),
                            visibility: Visibility::Hidden,
                            transform: Transform::from_xyz(0., 12., 0.).with_scale(Vec3::splat(0.5)),
                            ..default()
                        },
                    ));
                });
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct LightSpeed;
