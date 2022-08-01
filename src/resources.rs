use bevy::prelude::*;

pub struct Game {
    pub letter_index: usize,
    pub has_target: bool,
    pub base_speed: f32,
    pub max_words: usize,
    pub score: usize,
}

pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

pub struct Assets {
    pub font: Handle<Font>,
}

pub struct WordList(pub Vec<String>);
