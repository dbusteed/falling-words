use bevy::prelude::*;

#[derive(Component)]
pub struct FallingWord {
    pub speed: f32,
}

#[derive(Component)]
pub struct TargetWord;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameNode;

#[derive(Component)]
pub struct MenuNode;

#[derive(Component)]
pub struct GameStartBtn;

#[derive(Component)]
pub struct RestartBtn;