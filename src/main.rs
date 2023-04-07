use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::keyboard::KeyboardInput;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::{EntityInstance, LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_rapier2d::prelude::*;

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
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
    #[cfg(feature = "debug-mode")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugin(WorldInspectorPlugin::new());
    }
    app.add_state::<AppState>();
    app.add_system(setup_main.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_system(exit_main.in_schedule(OnExit(AppState::MainMenu)));
    app.add_system(position_camera_at_ship);
    app.add_system(movement_input);
    app.add_system(spawn_entity_instances);
    app.add_system(my_cursor_system);
    app.add_system(beam_input);
    app.add_system(boost_input);
    app.add_system(camera_follow_ship.in_set(OnUpdate(AppState::InGame)));
    app.add_system(button_interactions_system);
    app.run();
}

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct InteractLightBeam;

#[derive(Resource)]
struct LdtkImageHolder(Handle<Image>);

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct StartAdventureButton;

#[derive(Component)]
struct MainMenuUI;


fn setup_main(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((MainMenuUI, NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        ..default()
    })).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.), Val::Px(350.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Bob's Adventure",
                TextStyle {
                    font: asset_server.load("fonts/JollyLodger-Regular.ttf"),
                    font_size: 128.,
                    color: Color::hex("#FFF").unwrap(),
                },
            ));
            parent.spawn((StartAdventureButton, ButtonBundle {
                style: Style {
                    size: Size::UNDEFINED,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,

                    ..default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0,
                }),
                ..default()
            }, )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "New Adventure",
                    TextStyle {
                        font: asset_server.load("fonts/NanumMyeongjo-Regular.ttf"),
                        font_size: 40.,
                        ..default()
                    },
                ));
            });
        });
    });

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.camera_2d.clear_color = ClearColorConfig::Custom(Color::hex("1B0A28").unwrap());
    camera_bundle.projection.scale *= 0.50;

    commands.spawn((MainCamera, camera_bundle));
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level.ldtk"),
        ..Default::default()
    });
    // bug workaround: https://github.com/Trouv/bevy_ecs_ldtk/issues/111

    commands.insert_resource(LdtkImageHolder(asset_server.load("Laser Lvl 1.png")));
    commands.insert_resource(LevelSelection::Index(1));
}

fn exit_main(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_interactions_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &Children),
        (Changed<Interaction>, With<StartAdventureButton>)>,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, children)
    in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(LevelSelection::Index(0));
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                text.sections[0].value = "- New Adventure -".to_string();
            }
            Interaction::None => {
                text.sections[0].value = "New Adventure".to_string();
            }
        }
    }
}

fn spawn_entity_instances(
    mut commands: Commands,
    player_q: Query<(Entity, &EntityInstance, &Transform, &GlobalTransform), (Added<EntityInstance>, Without<Ship>)>,
    mut bob_ship_q: Query<&mut Transform, With<Ship>>,
    asset_server: Res<AssetServer>,
) {
    for
    (entity, instance, p_transform, global_transform)
    in player_q.iter() {
        match instance.identifier.as_ref() {
            "Player" => {
                if bob_ship_q.is_empty() {
                    println!("Creating Bob's Ship");
                    let bob_bundle = (
                        Ship,
                        SpriteBundle {
                            texture: asset_server.load("Bob's Ship.png"),
                            transform: *p_transform,
                            ..default()
                        },
                        RigidBody::Dynamic,
                        GravityScale(0.),
                        Velocity::zero(),
                        Speed(90.),
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
            _ => {}
        }
    }
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

fn boost_input(
    mut ship_q: Query<&mut Speed, With<Ship>>,
    key_input: Res<Input<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::LShift) {
        for mut speed in ship_q.iter_mut() {
            speed.0 += 70.;
        }
    } else if key_input.just_released(KeyCode::LShift) {
        for mut speed in ship_q.iter_mut() {
            speed.0 -= 70.;
        }
    }
}

fn beam_input(
    mut commands: Commands,
    beam_q: Query<Entity,
        With<InteractLightBeam>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        for beam in beam_q.iter() {
            commands.entity(beam).insert(Visibility::Visible);
        }
    } else {
        for beam in beam_q.iter() {
            commands.entity(beam).insert(Visibility::Hidden);
        }
    }
}

fn movement_input(
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
