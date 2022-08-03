use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub fn match_letter(keycode: &KeyCode) -> Option<&'static str> {
    match keycode {
        KeyCode::A => Some("a"),
        KeyCode::B => Some("b"),
        KeyCode::C => Some("c"),
        KeyCode::D => Some("d"),
        KeyCode::E => Some("e"),
        KeyCode::F => Some("f"),
        KeyCode::G => Some("g"),
        KeyCode::H => Some("h"),
        KeyCode::I => Some("i"),
        KeyCode::J => Some("j"),
        KeyCode::K => Some("k"),
        KeyCode::L => Some("l"),
        KeyCode::M => Some("m"),
        KeyCode::N => Some("n"),
        KeyCode::O => Some("o"),
        KeyCode::P => Some("p"),
        KeyCode::Q => Some("q"),
        KeyCode::R => Some("r"),
        KeyCode::S => Some("s"),
        KeyCode::T => Some("t"),
        KeyCode::U => Some("u"),
        KeyCode::V => Some("v"),
        KeyCode::W => Some("w"),
        KeyCode::X => Some("x"),
        KeyCode::Y => Some("y"),
        KeyCode::Z => Some("z"),
        _ => None,
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
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
