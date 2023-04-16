use std::time::Duration;
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_rapier2d::prelude::*;
use aggro::Aggro;
use animation::Animation;
use beams::{BeamUpEvent, InteractLightBeam, UnderBeamItems};
use camera::MainCamera;
use level::{Damage, DamageCollider};
use movement::Speed;
use ship::Ship;
use crate::audio::start_background_audio;
use crate::level::{AggroRange, Health, Inventory, Item, ResourceNameplate};
use crate::ui::{GameOverUI, InGameUI, IntroUI, inventory_interactions, inventory_ui, MainMenuUI, PanelMainText, PanelText};

mod ui;
mod level;
mod animation;
mod camera;
mod movement;
mod audio;
mod beams;
mod ship;
mod aggro;
mod damage;
mod cursor;

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
    #[cfg(feature = "debug-mode")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
    app.add_system(start_background_audio.on_startup());
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
    app.add_system(camera::camera_follow_ship.in_set(OnUpdate(AppState::InGame)));
    app.add_systems((damage::handle_collisions, beams::beam_up).chain().in_set(OnUpdate(AppState::InGame)));
    app.add_systems((aggro::handle_aggro, aggro::aggro_movement).chain().in_set(OnUpdate(AppState::InGame)));
    app.add_systems((damage::handle_collision_damage, damage::handle_damage).chain().in_set(OnUpdate(AppState::InGame)));
    app.add_system(ui::clean_up_ui::<InGameUI>.in_schedule(OnExit(AppState::InGame)));
    app.add_system(ui::setup_game_over.in_schedule(OnEnter(AppState::GameOver)));
    app.add_system(ui::menu_button_interactions_system.in_set(OnUpdate(AppState::GameOver)));
    app.add_system(ui::clean_up_ui::<GameOverUI>.in_schedule(OnExit(AppState::GameOver)));
    app.add_event::<BeamUpEvent>();
    app.add_system(inventory_ui.in_set(OnUpdate(AppState::InGame)));
    app.add_systems((ui::panel_text_update, inventory_interactions).chain().in_set(OnUpdate(AppState::InGame)));
    app.add_system(camera::position_camera_at_ship);
    app.add_system(movement::movement_input);
    app.add_system(level::spawn_entity_instances);
    app.add_system(cursor::my_cursor_system);
    app.add_system(beams::beam_input);
    app.add_system(beams::boost_input);
    app.add_system(animation::animation_system);
    app.run();
}

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


