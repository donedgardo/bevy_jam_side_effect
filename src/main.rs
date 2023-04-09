use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_rapier2d::prelude::*;
use animation::Animation;
use camera::MainCamera;
use movement::Speed;

mod ui;
mod level;
mod animation;
mod camera;
mod movement;

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Intro,
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
    app.add_system(setup_start_menu.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_system(camera::setup_main_camera.in_schedule(OnEnter(AppState::MainMenu)));
    app.add_system(ui::menu_button_interactions_system.in_set(OnUpdate(AppState::MainMenu)));
    app.add_system(ui::exit_main.in_schedule(OnExit(AppState::MainMenu)));
    app.add_system(ui::setup_intro.in_schedule(OnEnter(AppState::Intro)));
    app.add_system(ui::dialog_interaction_system.in_set(OnUpdate(AppState::Intro)));
    app.add_system(camera_follow_ship.in_set(OnUpdate(AppState::InGame)));
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
    beam_q: Query<Entity,
        With<InteractLightBeam>>,
    ship_q: Query<Entity, With<Ship>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for beam in beam_q.iter() {
            commands.entity(beam).insert(Visibility::Visible);
        }
        for ship in ship_q.iter() {
            animation::add_blinking_animation(&mut commands, ship);
        }
    }
    if mouse_input.just_released(MouseButton::Left) {
        for beam in beam_q.iter() {
            commands.entity(beam).insert(Visibility::Hidden);
        }
        for ship in ship_q.iter() {
            commands.entity(ship).remove::<Animation>();
        }
    }
}
