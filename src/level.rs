use std::slice::Iter;
use bevy::prelude::*;
use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec2;
use bevy::hierarchy::BuildChildren;
use bevy_ecs_ldtk::EntityInstance;
use bevy_rapier2d::dynamics::{GravityScale, RigidBody, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Sensor};
use benimator::FrameRate;

use crate::beams::InteractLightBeam;
use crate::animation::{Animation, AnimationState};
use crate::movement::Speed;
use crate::ship::Ship;

#[derive(Component, Clone)]
pub struct Item {
    pub description: String,
    pub texture: Handle<Image>,
}

#[derive(Component)]
pub struct ResourceNameplate;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(hp: f32) -> Self {
        Health {
            current: hp,
            max: hp,
        }
    }
}

#[derive(Component)]
pub struct Inventory {
    capacity: u32,
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            items: vec![],
        }
    }
    pub fn add(&mut self, item: &Item) {
        if self.items.len() >= self.capacity as usize { return; }
        self.items.push(item.clone());
    }

    pub fn iter(&self) -> Iter<'_, Item> {
        self.items.iter()
    }
}

#[derive(Component, Clone)]
pub struct Herbs;

#[derive(Component, Clone)]
pub struct Organism;

#[derive(Component, Clone)]
pub struct YellowOrganism;

#[derive(Component, Clone)]
pub struct Gold;

#[derive(Component, Clone)]
pub struct Element251;

#[derive(Component, Clone)]
pub struct Water;

#[derive(Component, Clone)]
pub struct WeaponArtifact;

#[derive(Component, Clone)]
pub struct ShieldArtifact;

#[derive(Component)]
pub struct AggroRange;

#[derive(Component)]
pub struct DestructiveLightBeam;

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
                        Health::new(10.),
                        Inventory::new(50),
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
                        let mut destructive_beam_translation = Transform::from(*global_transform);
                        destructive_beam_translation.translation.x += 8. * 10.;
                        destructive_beam_translation.translation.z += 1.;
                        let texture_handle = asset_server.load("destructive-beam-sheet.png");
                        let texture_atlas =
                            TextureAtlas::from_grid(texture_handle, Vec2::new(128., 48.0), 8, 1, None, None);
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        parent.spawn((DestructiveLightBeam,
                                      SpriteSheetBundle {
                                          texture_atlas: texture_atlas_handle,
                                          transform: destructive_beam_translation,
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
                    Herbs,
                    Item {
                        description: "Herbs:\nGain Life Support.\nSide Effects: Locals will come after you.".to_string(),
                        texture: asset_server.load("resources.png"),
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
            "Gold" => {
                let texture_handle = asset_server.load("resources-gold.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16., 16.), 1, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands.entity(entity).insert((
                    Gold,
                    Item {
                        description: "Gold:\nGreat, I definitely need this.\n.".to_string(),
                        texture: asset_server.load("resources-gold.png"),
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
                            text: Text::from_section("Gold", TextStyle {
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
            "Element251" => {
                let texture_handle = asset_server.load("resources-element-251.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16., 16.), 1, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands.entity(entity).insert((
                    Element251,
                    Item {
                        description: "Element251:\nRare element that unlocks advanced technology.\nSide Effects: ???.".to_string(),
                        texture: asset_server.load("resources-element-251.png"),
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
                            text: Text::from_section("Element251", TextStyle {
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
            "Water" => {
                let texture_handle = asset_server.load("resources-water.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16., 16.), 1, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands.entity(entity).insert((
                    Water,
                    Item {
                        description: "Water:\nEssential for survival.\nSide Effects: Taking water will anger locals.".to_string(),
                        texture: asset_server.load("resources-water.png"),
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
                            text: Text::from_section("Water", TextStyle {
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
            "WeaponArtifact" => {
                let texture_handle = asset_server.load("artifact.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16., 16.), 1, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands.entity(entity).insert((
                    WeaponArtifact,
                    Item {
                        description: "Weapon Artifact:\nRadioactive weapon, capable of destruction.\n\
                        Side Effects:\nConsume 2 gold, 1 element251, 3 water, 5 organisms to activate.".to_string(),
                        texture: asset_server.load("artifact.png"),
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
                            text: Text::from_section("Weapon Artifact", TextStyle {
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
            "ShieldArtifact" => {
                let texture_handle = asset_server.load("artifact-shield.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16., 16.), 1, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands.entity(entity).insert((
                    ShieldArtifact,
                    Item {
                        description: "Shield Artifact:\nReflective capabilities.\n\
                        Side Effects:\nConsumes 15 water, 1 element251, 3 herbs, 1 organisms to activate.".to_string(),
                        texture: asset_server.load("artifact-shield.png"),
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
                            text: Text::from_section("Shield Artifact", TextStyle {
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
            "Organism" => {
                let texture_handle = asset_server.load("organism-sheet.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(32., 32.), 12, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                let animation = Animation(benimator::Animation::from_indices(
                    0..=11,
                    FrameRate::from_fps(10.0),
                ));
                commands.entity(entity).insert((
                    Item {
                        description: "Organism:\nCan be genetically modified to work for you.\nSide Effect: Could turn against you.".to_string(),
                        texture: asset_server.load("organism.png"),
                    },
                    Organism,
                    Collider::ball(14.),
                    Sensor,
                    animation,
                    RigidBody::Dynamic,
                    GravityScale(0.),
                    Velocity::zero(),
                    Speed(80.),
                    AnimationState::default(),
                    ActiveEvents::COLLISION_EVENTS,
                    DamageCollider(1.),
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: *p_transform,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        ResourceNameplate,
                        Text2dBundle {
                            text: Text::from_section("Life Form", TextStyle {
                                font: asset_server.load("fonts/static/JetBrainsMono-Light.ttf"),
                                font_size: 16.,
                                ..default()
                            }),
                            visibility: Visibility::Hidden,
                            transform: Transform::from_xyz(0., 24., 0.).with_scale(Vec3::splat(0.5)),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Collider::ball(96.),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                        AggroRange
                    ));
                });
            }
            "Hostiles" => {
                let texture_handle = asset_server.load("organism-yellow-sheet.png");
                let texture_atlas =
                    TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(32., 32.), 12, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                let animation = Animation(benimator::Animation::from_indices(
                    0..=11,
                    FrameRate::from_fps(10.0),
                ));
                commands.entity(entity).insert((
                    Item {
                        description: "Organism:\nCan be genetically modified to work for you.\nSide Effect: Could turn against you.".to_string(),
                        texture: asset_server.load("organism-yellow.png"),
                    },
                    YellowOrganism,
                    Collider::ball(14.),
                    Sensor,
                    animation,
                    AnimationState::default(),
                    RigidBody::Dynamic,
                    GravityScale(0.),
                    Velocity::zero(),
                    Speed(80.),
                    ActiveEvents::COLLISION_EVENTS,
                    DamageCollider(2.),
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: *p_transform,
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        ResourceNameplate,
                        Text2dBundle {
                            text: Text::from_section("Life Form", TextStyle {
                                font: asset_server.load("fonts/static/JetBrainsMono-Light.ttf"),
                                font_size: 16.,
                                ..default()
                            }),
                            visibility: Visibility::Hidden,
                            transform: Transform::from_xyz(0., 24., 0.).with_scale(Vec3::splat(0.5)),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Collider::ball(106.),
                        Sensor,
                        ActiveEvents::COLLISION_EVENTS,
                        AggroRange
                    ));
                });
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct LightSpeed;

#[derive(Component)]
pub struct DamageCollider(pub f32);

#[derive(Component)]
pub struct Damage(pub f32, pub Timer);
