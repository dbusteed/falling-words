use bevy::{prelude::*, window::WindowResizeConstraints};
use iyes_loopless::prelude::*;

// we need these `mod`s so the plugins can
// do `use crate::blah::{blah}`
mod components;
mod resources;
mod util;

use resources::Assets;

mod main_menu;
use main_menu::MainMenuPlugin;

mod game;
use game::GamePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.17, 0.17, 0.17)))
        .insert_resource(WindowDescriptor {
            title: "Falling Words".to_string(),
            width: 400.0,
            height: 500.0,
            resizable: false,
            resize_constraints: WindowResizeConstraints {
                min_width: 400.0,
                max_width: 400.0,
                min_height: 500.0,
                max_height: 500.0,
            },
            ..default()
        })
        .add_loopless_state(AppState::MainMenu)
        .add_startup_system(global_setup_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

fn global_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    
    commands.insert_resource(Assets {
        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
    });
}
