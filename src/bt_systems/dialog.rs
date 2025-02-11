use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

use crate::{DialogStatement, ExitDialog};

/// Handle Esc key press (open exit dialog)
pub fn handle_esc_key(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut exit_dialog: ResMut<ExitDialog>,
    mut cmd: Commands,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        if let Key::Escape = ev.logical_key {
            if !exit_dialog.active {
                exit_dialog.active = true;

                cmd.spawn((
                    DialogStatement {},
                    Text2d::new("Exit the application? (Y/n)"),
                    TextFont {
                        font_size: 32.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 0.0, 10.0), // Center of the screen
                ));
            }
        }
    }
}

/// Handle Y / N input for exit dialog
pub fn handle_exit_dialog(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut exit_dialog: ResMut<ExitDialog>,
    mut cmd: Commands,
    query: Query<Entity, With<DialogStatement>>,
) {
    if !exit_dialog.active {
        return;
    }

    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }

        match &ev.logical_key {
            Key::Character(ch) if ch.eq_ignore_ascii_case("y") => {
                println!("Exiting the application.");
                std::process::exit(0);
            }
            Key::Character(ch) if ch.eq_ignore_ascii_case("n") => {
                exit_dialog.active = false;

                // Remove the exit dialog text
                for entity in query.iter() {
                    cmd.entity(entity).despawn();
                }
                println!("Exit canceled.");
            }
            _ => {}
        }
    }
}
