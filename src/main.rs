//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

pub mod menu;
pub mod game;
pub mod utils;

use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Game,
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Component, PartialEq, Eq, Clone)]
struct Nick(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(Difficulty::Normal)
        .insert_resource(Nick(String::from("Player")))
        .add_startup_system(setup)
        // Declare the game state, and set its startup value
        .add_state(GameState::Menu)
        // Adds the plugins for each state
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}


