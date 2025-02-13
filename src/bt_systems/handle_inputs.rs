use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

use crate::{BakeryTerminal, FocusedSection, OperatorMode};

#[derive(Event)]
pub struct Emitation(pub String);

/// Keyboard input handling (updates only the focused BakeryTerminal)
pub fn handle_text_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    focused_section: Res<FocusedSection>,
    mut sections: Query<&mut BakeryTerminal>,
    mut emit_command: EventWriter<Emitation>,
) {
    sections
        .iter_mut()
        .filter(|terminal| terminal.id == focused_section.0)
        .for_each(|mut bakery_terminal| {
            evr_kbd
                .read()
                .filter(|ev| ev.state == ButtonState::Pressed)
                .for_each(|ev| match &ev.logical_key {
                    Key::Enter => {
                        let input_text = bakery_terminal.input_buffer.clone();
                        bakery_terminal.history.push(input_text.clone());
                        emit_command.send(Emitation(input_text));
                        bakery_terminal.input_buffer.clear();
                    }
                    Key::Backspace => {
                        bakery_terminal.input_buffer.pop();
                    }
                    Key::Character(input) => {
                        if !input.chars().any(|c| c.is_control()) {
                            bakery_terminal.input_buffer.push_str(input);
                        }
                    }
                    Key::Space => {
                        bakery_terminal.input_buffer.push('_');
                    }
                    _ => {}
                });
        });
}

/// Switch focused section based on keyboard input
pub fn switch_section(
    input: Res<ButtonInput<KeyCode>>,
    mut focused_section: ResMut<FocusedSection>,
    mut sections: Query<(&BakeryTerminal, &mut OperatorMode)>,
) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    if input.just_pressed(KeyCode::Tab) {
        if shift {
            focused_section.decrement();
        } else {
            focused_section.increment();
        }
    }

    let next_section = focused_section.0;

    sections
        .iter_mut()
        .for_each(|(section, mut operator_mode)| {
            *operator_mode = if section.id == next_section {
                OperatorMode::Commander
            } else {
                OperatorMode::Observer
            };
        });
}

/// Update BakeryTerminal text content
pub fn display(mut sections: Query<(&BakeryTerminal, &mut Text)>) {
    sections.iter_mut().for_each(|(bakery_terminal, mut text)| {
        text.0 =
            bakery_terminal.history.join("\n") + &format!("\n> {}", bakery_terminal.input_buffer);
    });
}
