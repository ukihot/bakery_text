use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

use crate::{BakeryTerminal, FocusedSection, OperatorMode};

#[derive(Event)]
pub struct Emitation(pub String, pub u8);

impl Emitation {
    pub fn split_command(&self) -> (&str, Option<&str>, Option<&str>) {
        let parts: Vec<&str> = self.0.split('_').collect();
        let command = parts.first().unwrap_or(&"");
        let opt1 = parts.get(1).copied();
        let opt2 = parts.get(2).copied();
        (command, opt1, opt2)
    }
}

/// Keyboard input handling (updates only the focused BakeryTerminal)
pub fn handle_text_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    focused_section: Res<FocusedSection>,
    mut sections: Query<&mut BakeryTerminal>,
    mut emit_command: EventWriter<Emitation>,
) {
    sections
        .iter_mut()
        .filter(|term| term.id == focused_section.0)
        .for_each(|mut term| {
            evr_kbd
                .read()
                .filter(|ev| ev.state == ButtonState::Pressed)
                .for_each(|ev| match &ev.logical_key {
                    Key::Enter => {
                        let input_text = term.submit_input();
                        emit_command.send(Emitation(input_text, term.id));
                    }
                    Key::Backspace => {
                        term.remove_last_input();
                    }
                    Key::Character(input) => {
                        if !input.chars().any(|c| c.is_control()) {
                            term.add_input(input);
                        }
                    }
                    Key::Space => {
                        term.add_input("_");
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
        let increment = |section: &mut FocusedSection| {
            section.increment();
            if section.0 == 9 || section.0 == 11 {
                section.increment();
            }
        };

        let decrement = |section: &mut FocusedSection| {
            section.decrement();
            if section.0 == 9 || section.0 == 11 {
                section.decrement();
            }
        };

        if shift {
            decrement(&mut focused_section);
        } else {
            increment(&mut focused_section);
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
pub fn presenter(mut sections: Query<(&BakeryTerminal, &mut Text)>) {
    sections.iter_mut().for_each(|(term, mut text)| {
        text.0 = term.history.join("\n") + &format!("\n> {}", term.input_buffer);
    });
}
