use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::hierarchy::{BuildChildren, Children, DespawnRecursiveExt};
use bevy_ecs_ldtk::LevelSelection;
use crate::AppState;
use crate::beams::{BeamUpEvent, UnderBeamItems};
use crate::level::{Element251, Gold, Health, Herbs, Item, LightSpeed, Organism, ShieldArtifact, Water, WeaponArtifact, YellowOrganism};
use crate::ship::Ship;

pub fn create_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
            })).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "New Game",
                    TextStyle {
                        font: asset_server.load("fonts/static/JetBrainsMono-Regular.ttf"),
                        font_size: 40.,
                        ..default()
                    },
                ));
            });
        });
    });
}

#[derive(Component)]
pub struct MainMenuUI;

pub fn clean_up_ui<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct GameOverUI;

pub fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LevelSelection::Index(1));
    commands.spawn((GameOverUI, NodeBundle {
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
                "Game Over!",
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
            })).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "New Game",
                    TextStyle {
                        font: asset_server.load("fonts/static/JetBrainsMono-Regular.ttf"),
                        font_size: 40.,
                        ..default()
                    },
                ));
            });
        });
    });
}

pub fn menu_button_interactions_system(
    mut interaction_query: Query<(&Interaction, &Children),
        (Changed<Interaction>, With<StartAdventureButton>)>,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                next_state.set(AppState::Intro);
            }
            Interaction::Hovered => {
                text.sections[0].value = "- New Game -".to_string();
            }
            Interaction::None => {
                text.sections[0].value = "New Game".to_string();
            }
        }
    }
}

#[derive(Resource)]
pub struct DialogState(pub Vec<String>);

#[derive(Component)]
struct DialogText;

#[derive(Component)]
pub struct DialogBox;

#[derive(Component)]
struct DialogContinueButton;

pub fn dialog_interaction_system(
    mut interaction_query: Query<(&Interaction, &Children, &mut Style),
        (Changed<Interaction>, With<DialogBox>)>,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
    mut dialog_state: ResMut<DialogState>,
) {
    for (interaction, children, mut style) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                dialog_state.0.pop();
                text.sections[0].value =
                    dialog_state.0.last().unwrap_or(&"".to_string()).to_string();
                if dialog_state.0.is_empty() { next_state.set(AppState::InGame) };
            }
            Interaction::None => {
                style.border = UiRect::all(Val::Px(0.));
                text.sections[0].value =
                    dialog_state.0.last().unwrap_or(&"".to_string()).to_string();
            }
            Interaction::Hovered => {
                style.border = UiRect::all(Val::Px(2.));
            }
        }
    }
}


#[derive(Component)]
pub struct IntroUI;

pub fn load_level(
    mut commands: Commands,
) {
    commands.insert_resource(LevelSelection::Index(0));
}


pub fn setup_intro(
    mut commands: Commands,
    light_query: Query<Entity, With<LightSpeed>>,
    asset_server: Res<AssetServer>,
    org_query: Query<Entity, With<Item>>,
) {
    for entity in org_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let dialog = DialogState(vec![
        "\"PS: Ship is fixed with 'wasd' for travel, whatever that means\"".to_string(),
        "\"For science!\"\n- Bob.".to_string(),
        "\"With this new material I can now cancel the effects of inertia\nand safely travel at light speeds!!\"".to_string(),
        "\"Its been a journey but I finally found material 251 on the planet 3.\"".to_string(),
        "Journal Entry: Day 1 - Click".to_string(),
    ]);
    commands.insert_resource(dialog);
    for entity in light_query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
    commands.spawn((IntroUI, NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        ..default()
    })).with_children(|parent| {
        parent.spawn((DialogBox, ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(80.), Val::Px(200.)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::hex("#43374F").unwrap()),
            ..default()
        })).with_children(|parent| {
            parent.spawn((DialogText, TextBundle::from_section(
                "".to_string(),
                TextStyle {
                    font: asset_server.load("fonts/static/JetBrainsMono-Regular.ttf"),
                    font_size: 32.,
                    ..default()
                },
            ).with_style(Style {
                max_size: Size::new(Val::Px(500.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })));
        });
    });
}

#[derive(Component)]
pub struct StartAdventureButton;

#[derive(Component)]
pub struct InGameUI;

#[derive(Component)]
pub struct PanelText;

#[derive(Component)]
pub struct InventoryPanel;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct InventoryText;

#[derive(Component)]
pub struct InventoryButton;

#[derive(Resource)]
pub struct PanelMainText(pub String);

pub fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PanelMainText("\"There is a powerful artifact in this world.\n\
    Where could it be?\"\n\
    -Bob".to_string()));
    commands.spawn((InGameUI, NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        ..default()
    })).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(130.)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(25.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(27. / 255., 10. / 255., 40. / 255., 0.9).into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn((HealthText, TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/static/JetBrainsMono-Light.ttf"),
                        font_size: 18.,
                        ..default()
                    })));
            });
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                    min_size: Size::new(Val::Px(300.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                background_color: Color::rgba(27. / 255., 10. / 255., 40. / 255., 0.9).into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn((PanelText, TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/static/JetBrainsMono-Light.ttf"),
                        font_size: 18.,
                        ..default()
                    })));
            });
            parent.spawn((InventoryPanel, NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(25.), Val::Percent(100.)),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                background_color: Color::rgba(27. / 255., 10. / 255., 40. / 255., 0.9).into(),
                ..default()
            }));
        });
    });
}

pub fn inventory_interactions(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, Entity, &Item), (Changed<Interaction>, With<InventoryButton>)>,
    mut panel_main_text: ResMut<PanelMainText>,
    mut health_q: Query<&mut Health, With<Ship>>,
    herb_q: Query<&Herbs>,
) {
    for (interaction, entity, item) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if herb_q.get(entity).is_ok() {
                    for mut health in health_q.iter_mut() {
                        if health.current < health.max {
                            health.current += 1.;
                        }
                    }
                }
                commands.entity(entity).despawn_recursive();
                panel_main_text.0 = "".to_string();
            }
            Interaction::None => {
                panel_main_text.0 = "".to_string();
            }
            Interaction::Hovered => {
                panel_main_text.0 = format!("{}\nLeft-click to use.", item.description);
            }
        }
    }
}

pub fn health_ui(
    health_q: Query<&Health, (Or<(Changed<Health>, Added<Health>)>, With<Ship>)>,
    mut text_q: Query<&mut Text, With<HealthText>>,
) {
    for health in health_q.iter() {
        for mut text in text_q.iter_mut() {
            text.sections[0].value = format!("Life Support: {}/{}", health.current, health.max);
        }
    }
}

pub fn inventory_ui(
    mut commands: Commands,
    mut beam_up_event: EventReader<BeamUpEvent>,
    inventory_panel_q: Query<Entity, With<InventoryPanel>>,
    item_q: Query<&Item>,
    herb_q: Query<&Herbs>,
    org_q: Query<&Organism>,
    yellow_q: Query<&YellowOrganism>,
    gold_q: Query<&Gold>,
    element_q: Query<&Element251>,
    water_q: Query<&Water>,
    weapon_q: Query<&WeaponArtifact>,
    shield_q: Query<&ShieldArtifact>,
) {
    if let Ok(inventory_panel) = inventory_panel_q.get_single() {
        for ev in beam_up_event.iter() {
            if let Ok(item) = item_q.get(ev.0) {
                let id = commands.spawn((
                    InventoryButton,
                    item.clone(),
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::new(Val::Px(2.), Val::Px(2.), Val::Px(2.), Val::Px(2.)),
                            size: Size::new(Val::Px(24.), Val::Px(24.)),
                            ..default()
                        },
                        image: UiImage::new(item.texture.clone()),
                        ..default()
                    })).id();
                if let Ok(element) = herb_q.get(ev.0) {
                    commands.entity(id).insert(element.clone());
                }
                if let Ok(element) = org_q.get(ev.0) {
                    commands.entity(id).insert(element.clone());
                }
                if let Ok(element) = yellow_q.get(ev.0) {
                    commands.entity(id).insert(element.clone());
                }
                if let Ok(element) = gold_q.get(ev.0) {
                    commands.entity(id).insert(element.clone());
                }
                if let Ok(element) = element_q.get(ev.0) {
                    commands.entity(id).insert(element.clone());
                }
                if let Ok(element) = water_q.get(ev.0) {
                    commands.entity(id).insert(element.clone());
                }
                if let Ok(element) = weapon_q.get(ev.0) {
                    commands.entity(id).insert(element.clone());
                }
                if let Ok(element) = shield_q.get(ev.0) {
                    commands.entity(id).insert(element.clone());
                }
                commands.entity(ev.0).despawn_recursive();
                commands.entity(inventory_panel).add_child(id);
            }
        }
    }
}

pub fn panel_text_update(
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
