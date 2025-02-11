use bevy::prelude::*;

use super::bakery_terminal::Emitation;

pub fn handle_commands(mut events: EventReader<Emitation>) {
    for ev in events.read() {
        eprintln!("Command: {}", ev.0);
    }
}
