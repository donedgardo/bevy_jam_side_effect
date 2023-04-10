use std::time::Duration;
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_rapier2d::prelude::*;
use animation::Animation;
use camera::MainCamera;
use level::{Damage, DamageCollider};
use movement::Speed;
use crate::audio::start_background_audio;
use crate::level::{AggroRange, Health, Inventory, Item, ResourceNameplate};
use crate::ui::{GameOverUI, InGameUI, IntroUI, inventory_interactions, inventory_ui, MainMenuUI, PanelMainText, PanelText};

mod ui;
mod level;
mod animation;
mod camera;
mod movement;
mod audio;

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Intro,
    InGame,
    GameOver,
}


fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        })
        .set(WindowPlugin {
            primary_window: Option::from(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }));
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(LdtkPlugin);
    app.add_plugin(AudioPlugin);
    app.add_system(start_background_audio.on_startup());
    #[cfg(feature = "debug-mode")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
    app.add_state::<AppState>();
    app.insert_resource(UnderBeamItems(vec![]));
    app.add_system(setup_start_menu.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_system(camera::setup_main_camera.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_system(ui::menu_button_interactions_system.in_set(OnUpdate(AppState::MainMenu)));
    app.add_system(ui::clean_up_ui::<MainMenuUI>.in_schedule(OnExit(AppState::MainMenu)));
    app.add_system(ui::setup_intro.in_schedule(OnEnter(AppState::Intro)));
    app.add_system(ui::dialog_interaction_system.in_set(OnUpdate(AppState::Intro)));
    app.add_systems((ui::clean_up_ui::<IntroUI>, ui::load_level).chain().in_schedule(OnExit(AppState::Intro)));
    app.add_system(ui::setup_game_ui.in_schedule(OnEnter(AppState::InGame)));
    app.add_system(ui::health_ui.in_set(OnUpdate(AppState::InGame)));
    app.add_system(camera_follow_ship.in_set(OnUpdate(AppState::InGame)));
    app.add_systems((handle_collisions, beam_up).chain().in_set(OnUpdate(AppState::InGame)));
    app.add_systems((handle_aggro, aggro_movement).chain().in_set(OnUpdate(AppState::InGame)));
    app.add_systems((handle_collision_damage, handle_damage).chain().in_set(OnUpdate(AppState::InGame)));
    app.add_system(ui::clean_up_ui::<InGameUI>.in_schedule(OnExit(AppState::InGame)));
    app.add_system(ui::setup_game_over.in_schedule(OnEnter(AppState::GameOver)));
    app.add_system(ui::menu_button_interactions_system.in_set(OnUpdate(AppState::GameOver)));
    app.add_system(ui::clean_up_ui::<GameOverUI>.in_schedule(OnExit(AppState::GameOver)));
    app.add_event::<BeamUpEvent>();
    app.add_system(inventory_ui.in_set(OnUpdate(AppState::InGame)));
    app.add_systems((panel_text_update, inventory_interactions).chain().in_set(OnUpdate(AppState::InGame)));
    app.add_system(position_camera_at_ship);
    app.add_system(movement::movement_input);
    app.add_system(level::spawn_entity_instances);
    app.add_system(my_cursor_system);
    app.add_system(beam_input);
    app.add_system(boost_input);
    app.add_system(animation::animation_system);
    app.run();
}


#[derive(Component)]
pub struct Ship;

#[derive(Component)]
struct InteractLightBeam;

#[derive(Resource)]
struct LdtkImageHolder(Handle<Image>);

#[derive(Resource)]
struct UnderBeamItems(Vec<Entity>);

#[derive(Component)]
struct Aggro(Entity);

fn handle_aggro(
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

fn handle_collision_damage(
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

fn handle_damage(
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

fn aggro_movement(
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

fn handle_collisions(
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

fn panel_text_update(
    mut panel_query: Query<&mut Text, With<PanelText>>,
    under_beam: Res<UnderBeamItems>,
    herb_query: Query<&Item>,
    panel_main_text: Res<PanelMainText>,
) {
    for mut text in panel_query.iter_mut() {
        if under_beam.0.is_empty() {
            text.sections[0].value = panel_main_text.0.clone();
        } else {
            let item = under_beam.0.last().unwrap();
            if let Ok(herb) = herb_query.get(*item) {
                text.sections[0].value = format!("{}\nHit Spacebar to beam up.", herb.description);
            }
        }
    }
}

fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level.ldtk"),
        ..Default::default()
    });
    // bug workaround: https://github.com/Trouv/bevy_ecs_ldtk/issues/111
    commands.insert_resource(LdtkImageHolder(asset_server.load("Laser Lvl 1.png")));
    commands.insert_resource(LevelSelection::Index(1));
    ui::create_main_menu(&mut commands, &asset_server);
}

fn position_camera_at_ship(
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

fn camera_follow_ship(
    ship_q: Query<&Transform, (With<Ship>, Changed<Transform>, Without<MainCamera>)>,
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
) {
    for ship_transform in ship_q.iter() {
        camera_move_to(&mut camera_q, ship_transform);
    }
}

fn my_cursor_system(
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

// TODO: seperate animation and physics logic into different system, including beam input system
fn boost_input(
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

fn beam_input(
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

pub struct BeamUpEvent(Entity);

fn beam_up(
    mut under_beam: ResMut<UnderBeamItems>,
    input: Res<Input<KeyCode>>,
    item_query: Query<&Item>,
    mut ev_beam_up: EventWriter<BeamUpEvent>,
    mut inventory_query: Query<&mut Inventory, With<Ship>>,
) {
    if !input.just_pressed(KeyCode::Space) || under_beam.0.is_empty() { return; }
    let beamed_entity = under_beam.0.pop().unwrap();
    if let Ok(herb) = item_query.get(beamed_entity) {
        if let Ok(mut inventory) = inventory_query.get_single_mut() {
            inventory.add(herb);
            ev_beam_up.send(BeamUpEvent(beamed_entity));
        }
    }
}


