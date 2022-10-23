use bevy::prelude::*;

use crate::{utils::despawn_screen, GameState, TEXT_COLOR, Difficulty};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Main,
    Difficulty,
    Load,
    Disabled,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Main)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
            // main
            .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(main_menu_setup))
            .add_system_set(
                SystemSet::on_exit(MenuState::Main).with_system(despawn_screen::<OnMainMenuScreen>),
            )
            // difficulty
            .add_system_set(
                SystemSet::on_enter(MenuState::Difficulty).with_system(difficulty_menu_setup),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Difficulty)
                    .with_system(despawn_screen::<OnDifficultyMenuScreen>),
            )
            .add_system_set(SystemSet::on_update(MenuState::Difficulty).with_system(difficulty_button::<Difficulty>))
            // load
            .add_system_set(SystemSet::on_enter(MenuState::Load).with_system(load_menu_setup))
            .add_system_set(
                SystemSet::on_exit(MenuState::Load).with_system(despawn_screen::<OnLoadMenuScreen>),
            )
            // buttons
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(menu_action)
                    .with_system(button_system),
            );
    }
}

fn menu_setup(mut menu_state: ResMut<State<MenuState>>) {
    let _ = menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::ORANGE_RED.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Title
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Crabber",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
            // buttons
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Play)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "New Game",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Load)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Load Game",
                        button_text_style.clone(),
                    ));
                });
        });
}

fn difficulty_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands.spawn_bundle(NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::ORANGE_RED.into(),
        ..default()
    })
    .insert(OnDifficultyMenuScreen)
    .with_children(|parent| {
        // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
        // use the default value, `FlexDirection::Row`, from left to right.
        parent.spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::ORANGE_RED.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Difficulty",
                button_text_style.clone(),
            ));
            for difficulty in [
                Difficulty::Easy,
                Difficulty::Normal,
                Difficulty::Hard,
            ] {
                let mut entity = parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        ..button_style.clone()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                });
                entity.insert(MenuButtonAction::StartGame)
                .insert(difficulty).with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        format!("{difficulty:?}"),
                        button_text_style.clone()
                    ));
                });
            }
        });
        parent.spawn_bundle(ButtonBundle {
            style: button_style,
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(MenuButtonAction::BackToMainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section("Back", button_text_style));
        });
    });
}
fn load_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::ORANGE_RED.into(),
            ..default()
        })
        .insert(OnLoadMenuScreen)
        .with_children(|parent| {
            // Title
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Load Game",
                    TextStyle {
                        font: font.clone(),
                        font_size: 60.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
            // buttons
            parent.spawn_bundle(ButtonBundle {
                style: button_style,
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(MenuButtonAction::BackToMainMenu)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section("Back", button_text_style));
            });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Play => {
                    menu_state.set(MenuState::Difficulty).unwrap();
                }
                MenuButtonAction::StartGame => {
                    menu_state.set(MenuState::Disabled).unwrap();
                    game_state.set(GameState::Game).unwrap();
                }
                MenuButtonAction::Load => {
                    menu_state.set(MenuState::Load).unwrap();
                }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::Main).unwrap();
                }
            }
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::None => NORMAL_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::Clicked => PRESSED_BUTTON.into(),
        }
    }
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
fn difficulty_button<T: Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T), (Changed<Interaction>, With<Button>)>,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting) in &interaction_query {
        if *interaction == Interaction::Clicked && *setting != *button_setting {
            *setting = *button_setting;
        }
    }
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the difficulty menu screen
#[derive(Component)]
struct OnDifficultyMenuScreen;

// Tag component used to tag entities added on the load menu screen
#[derive(Component)]
struct OnLoadMenuScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    StartGame,
    Load,
    BackToMainMenu,
}
