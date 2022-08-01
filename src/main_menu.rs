use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::AppState;
use crate::components::{GameStartBtn, MenuNode};
use crate::resources::Assets;
use crate::util::{button_system, on_btn_interact, NORMAL_BUTTON};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MainMenu, setup_menu_system)
            .add_exit_system(AppState::MainMenu, remove_menu_system)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::MainMenu)
                    .with_system(btn_ingame.run_if(on_btn_interact::<GameStartBtn>))
                    .with_system(button_system)
                    .into(),
            );
    }
}

fn setup_menu_system(mut commands: Commands, assets: Res<Assets>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(225.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Start Game",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..default()
            });
        })
        .insert(MenuNode)
        .insert(GameStartBtn);
}

fn remove_menu_system(mut commands: Commands, query: Query<Entity, With<MenuNode>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn btn_ingame(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::InGame));
}
