use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

use crate::{bt_resources::plant::*, BakeryTerminal, FocusedSection};

#[derive(Event)]
pub struct Emitation(pub String);

pub fn install_bakery(mut cmd: Commands, window_query: Query<&Window>) {
    // Get window size
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    // Spacing for horizontal and vertical placement
    let horizontal_spacing = 500.0;
    let vertical_spacing = 200.0;

    // Fixed number of BakeryTerminals
    let num_terminals = 12;

    // Calculate the number of columns and rows based on num_terminals
    let num_columns = (num_terminals as f32).sqrt().ceil() as usize; // Number of columns
    let num_rows = (num_terminals + num_columns - 1) / num_columns; // Number of rows

    // Calculate positions for each BakeryTerminal
    for i in 0..num_terminals {
        let row = i / num_columns;
        let col = i % num_columns;

        // Adjust position based on center alignment
        let x = (col as f32 - (num_columns as f32 / 2.0 - 0.5)) * horizontal_spacing;
        let y = (row as f32 - (num_rows as f32 / 2.0 - 0.5)) * vertical_spacing;

        // Adjust position based on window size
        let adjusted_x = (window_width / 2.0) + x;
        let adjusted_y = (window_height / 2.0) + y;

        // Add background rectangle
        cmd.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.4),
                custom_size: Some(Vec2::new(400.0, 150.0)), // Background size
                ..Default::default()
            },
            Transform::from_xyz(adjusted_x, adjusted_y, -1.0),
        ));

        // Add BakeryTerminal text
        cmd.spawn((
            BakeryTerminal {
                id: i as u8,
                input_buffer: String::new(),
                history: vec![
                    format!("BakeryTerminal Terminal {i}"),
                    "Type 'help' for cmd.".to_string(),
                ],
            },
            Text2d::new(format!("BakeryTerminal Terminal {i}\nType 'help' for cmd.")),
            TextFont {
                font_size: 24.0,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Left),
            TextColor(Color::WHITE),
            Transform::from_xyz(adjusted_x, adjusted_y, 0.0), // Ensure text appears above background
        ));
    }
}

/// Keyboard input handling (updates only the focused BakeryTerminal)
pub fn text_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    focused_section: Res<FocusedSection>,
    mut sections: Query<&mut BakeryTerminal>,
    mut emit_command: EventWriter<Emitation>,
) {
    for mut bakery_terminal in sections.iter_mut() {
        if bakery_terminal.id != focused_section.0 {
            continue;
        }

        for ev in evr_kbd.read() {
            if ev.state == ButtonState::Released {
                continue;
            }
            match &ev.logical_key {
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
                    if input.chars().any(|c| c.is_control()) {
                        continue;
                    }
                    bakery_terminal.input_buffer.push_str(input);
                }
                _ => {}
            }
        }
    }
}

/// Switch focus using the Tab key
pub fn activate_section(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut focused_section: ResMut<FocusedSection>,
    mut purchasing: ResMut<PurchasingSection>,
    mut pantry: ResMut<PantrySection>,
    mut mixing: ResMut<MixingSection>,
    mut cooling: ResMut<CoolingSection>,
    mut shaping: ResMut<ShapingSection>,
    mut baking: ResMut<BakingSection>,
    mut packaging: ResMut<PackagingSection>,
    mut quality: ResMut<QualitySection>,
    mut stockroom: ResMut<StockroomSection>,
    mut waste: ResMut<WasteSection>,
) {
    for ev in evr_kbd.read() {
        if !ev.state.is_pressed() {
            continue;
        }

        if ev.logical_key == Key::Tab {
            focused_section.increment();
        }
    }

    // focused_sectionの値でoparatormodeを変更する
    match focused_section.0 {
        0 => {
            purchasing.activate();
            waste.deactivate();
        }
        1 => {
            pantry.activate();
            purchasing.deactivate();
        }
        2 => {
            mixing.activate();
            pantry.deactivate();
        }
        3 => {
            cooling.activate();
            mixing.deactivate();
        }
        4 => {
            shaping.activate();
            cooling.deactivate();
        }
        5 => {
            baking.activate();
            shaping.deactivate();
        }
        6 => {
            packaging.activate();
            baking.deactivate();
        }
        7 => {
            quality.activate();
            packaging.deactivate();
        }
        8 => {
            stockroom.activate();
            quality.deactivate();
        }
        9 => {
            waste.activate();
            stockroom.deactivate();
        }
        _ => {}
    }
}

/// Update BakeryTerminal text content
pub fn display(mut sections: Query<(&BakeryTerminal, &mut Text2d)>) {
    for (bakery_terminal, mut text) in sections.iter_mut() {
        text.0 =
            bakery_terminal.history.join("\n") + &format!("\n> {}", bakery_terminal.input_buffer);
    }
}
