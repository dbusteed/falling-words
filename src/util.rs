use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub fn match_letter(keycode: &KeyCode) -> &'static str {
    match keycode {
        KeyCode::A => "a",
        KeyCode::B => "b",
        KeyCode::C => "c",
        KeyCode::D => "d",
        KeyCode::E => "e",
        KeyCode::F => "f",
        KeyCode::G => "g",
        KeyCode::H => "h",
        KeyCode::I => "i",
        KeyCode::J => "j",
        KeyCode::K => "k",
        KeyCode::L => "l",
        KeyCode::M => "m",
        KeyCode::N => "n",
        KeyCode::O => "o",
        KeyCode::P => "p",
        KeyCode::Q => "q",
        KeyCode::R => "r",
        KeyCode::S => "s",
        KeyCode::T => "t",
        KeyCode::U => "u",
        KeyCode::V => "v",
        KeyCode::W => "w",
        KeyCode::X => "x",
        KeyCode::Y => "y",
        KeyCode::Z => "z",
        _ => "_",
    }
}

pub fn on_btn_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => { *color = PRESSED_BUTTON.into(); }
            Interaction::Hovered => { *color = HOVERED_BUTTON.into(); }
            Interaction::None => { *color = NORMAL_BUTTON.into(); }
        }
    }
}