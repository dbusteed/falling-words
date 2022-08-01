use bevy::ecs::event::Events;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::{thread_rng, Rng};
use std::fs;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use super::AppState;
use crate::components::{FallingWord, GameNode, RestartBtn, ScoreText, TargetWord};
use crate::resources::{Assets, Game, WindowSize, WordList};
use crate::util::{button_system, match_letter, on_btn_interact, NORMAL_BUTTON};

struct LetterEvent(&'static str);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup_game_system)
            .add_event::<LetterEvent>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(spawn_words_system)
                    .with_system(attack_target_system)
                    .with_system(score_text_system)
                    .with_system(update_position_system)
                    .with_system(keyboard_listen_system)
                    .with_system(find_target_system)
                    .into(),
            )
            // .add_plugin(LogDiagnosticsPlugin::default())
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_enter_system(AppState::GameOver, setup_gameover_system)
            .add_exit_system(AppState::GameOver, cleanup_game)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::GameOver)
                    .with_system(btn_restart.run_if(on_btn_interact::<RestartBtn>))
                    .with_system(button_system)
                    .into(),
            );
    }
}

fn setup_game_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    assets: Res<Assets>,
) {
    // game data resource
    commands.insert_resource(Game {
        letter_index: 0,
        has_target: false,
        score: 0,
        max_words: 2,
        base_speed: 0.3,
    });

    // wordlist resource
    let words = fs::read_to_string("./assets/wordlist.txt").expect("Unable to find wordlist!");
    let wordlist = words
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    commands.insert_resource(WordList(wordlist));

    // window size resource
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    commands.insert_resource(WindowSize {
        width: win_w,
        height: win_h,
    });

    // game UI
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(3.),
                    left: Val::Px(5.),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 24.0,
                            color: Color::LIME_GREEN,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 24.0,
                            color: Color::LIME_GREEN,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(ScoreText)
        .insert(GameNode);
}

fn spawn_words_system(
    mut commands: Commands,
    game: Res<Game>,
    wordlist: Res<WordList>,
    window_size: Res<WindowSize>,
    assets: Res<Assets>,
    query: Query<&FallingWord>,
) {
    let n_words = query.iter().len();
    if n_words < game.max_words {
        let word = &wordlist.0[thread_rng().gen_range(0..wordlist.0.len())];

        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(-10.),
                        right: Val::Px(window_size.width / 2.)
                            + thread_rng().gen_range(-80..100) as f32,
                        ..default()
                    },
                    ..default()
                },
                text: Text {
                    sections: word
                        .chars()
                        .map(|ch| TextSection {
                            value: ch.to_string(),
                            style: TextStyle {
                                font: assets.font.clone(),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        })
                        .collect::<Vec<TextSection>>(),
                    ..default()
                },
                ..default()
            })
            .insert(FallingWord {
                speed: thread_rng().gen_range((game.base_speed - 0.15)..=(game.base_speed + 0.15)),
            })
            .insert(GameNode);
    }
}

fn update_position_system(
    mut commands: Commands,
    window: Res<WindowSize>,
    mut query: Query<(Entity, &mut Style, &FallingWord)>,
) {
    for (ent, mut style, word) in query.iter_mut() {
        style.position.top += word.speed;

        if let Val::Px(pos) = style.position.top {
            if pos >= window.height {
                commands.insert_resource(NextState(AppState::GameOver));
                commands.entity(ent).despawn();
            }
        }
    }
}

fn find_target_system(
    mut commands: Commands,
    mut letter_evt: EventReader<LetterEvent>,
    mut game: ResMut<Game>,
    query: Query<(Entity, &Text, &Style), With<FallingWord>>,
) {
    if !game.has_target {
        let mut entity_first_letters = query
            .iter()
            .map(|(e, t, s)| {
                if let Val::Px(pos) = s.position.top {
                    (e, t.sections[0].value.to_string(), pos)
                } else {
                    (e, t.sections[0].value.to_string(), 0.0)
                }
            })
            .collect::<Vec<(Entity, String, f32)>>();
        
        entity_first_letters.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        for ev in letter_evt.iter() {
            for (ent, first_letter, _pos) in entity_first_letters.iter() {
                if ev.0 == first_letter {
                    game.has_target = true;
                    game.letter_index = 1;
                    commands.entity(*ent).insert(TargetWord);
                    break;
                }
            }
        }
    }
}

fn attack_target_system(
    mut commands: Commands,
    mut letters: ResMut<Events<LetterEvent>>,
    mut game: ResMut<Game>,
    mut query: Query<(Entity, &mut Text), With<TargetWord>>,
) {
    if let Ok((ent, mut text)) = query.get_single_mut() {
        if game.letter_index == 1 {
            for i in 0..text.sections.len() {
                text.sections[i].style.color = Color::rgba(1., 0., 0., 1.);
            }
            text.sections[0].style.color = Color::rgba(1., 0., 0., 0.07);
        }

        for ev in letters.drain() {
            if ev.0 == text.sections[game.letter_index].value {
                text.sections[game.letter_index].style.color = Color::rgba(1., 0., 0., 0.07);
                game.letter_index += 1;

                if game.letter_index >= text.sections.len() {
                    commands.entity(ent).despawn();
                    game.has_target = false;

                    // update the score and the game params
                    game.score += 1;
                    game.max_words = (game.score / 10) + 2;
                    game.base_speed = ((game.score as f32 / 10.) * 0.05) + 0.3;
                }
            }
        }
    }
}

fn score_text_system(game: Res<Game>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = game.score.to_string();
    }
}

fn setup_gameover_system(mut commands: Commands, assets: Res<Assets>) {
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
        .with_children(|button| {
            button.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Restart",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    default(),
                ),
                ..default()
            });
        })
        .insert(RestartBtn)
        .insert(GameNode);
}

fn btn_restart(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::InGame));
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameNode>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn keyboard_listen_system(kb: Res<Input<KeyCode>>, mut letter_evt: EventWriter<LetterEvent>) {
    let mut letter: &str = "_";
    if let Some(keycode) = kb.get_just_pressed().last() {
        letter = match_letter(keycode);
    }

    if letter != "_" {
        letter_evt.send(LetterEvent(letter));
    }
}
