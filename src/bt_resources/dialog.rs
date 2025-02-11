use bevy::prelude::*;

#[derive(Resource)]
pub struct ExitDialog {
    pub active: bool, // true if the exit dialog is displayed
}
