use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::hierarchy::{BuildChildren, Children, DespawnRecursiveExt};
use crate::AppState;
use crate::level::LightSpeed;

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
}

#[derive(Component)]
pub struct MainMenuUI;

pub fn exit_main(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
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
                text.sections[0].value = "- New Adventure -".to_string();
            }
            Interaction::None => {
                text.sections[0].value = "New Adventure".to_string();
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
    mut interaction_query: Query<(&Interaction, &Children),
        (Changed<Interaction>, With<DialogBox>)>,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
    mut dialog_state: ResMut<DialogState>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                // next_state.set(AppState::InGame);
                dialog_state.0.pop();
                text.sections[0].value =
                    dialog_state.0.last().unwrap_or(&"".to_string()).to_string();
            }
            Interaction::None => {
                text.sections[0].value =
                    dialog_state.0.last().unwrap_or(&"".to_string()).to_string();
            }
            _ => {}
        }
    }
}


pub fn setup_intro(
    mut commands: Commands,
    light_query: Query<Entity, With<LightSpeed>>,
    asset_server: Res<AssetServer>,
) {
    let dialog = DialogState(vec![
        "For science!\n- Bob.".to_string(),
        "With this new material I can now cancel the effects of inertia\nand safely travel at light speeds!!".to_string(),
        "Its been a journey but I finally found material 251 on the planet 3.".to_string(),
        "Journal Entry: Day 1".to_string(),
    ]);
    commands.insert_resource(dialog);
    for entity in light_query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
    commands.spawn(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
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
                    font: asset_server.load("fonts/NanumMyeongjo-Regular.ttf"),
                    font_size: 32.,
                    ..default()
                },
            )));
        });
    });
}

#[derive(Component)]
pub struct StartAdventureButton;
