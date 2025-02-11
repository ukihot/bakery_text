use bevy::prelude::*;

/// BakeryTerminal component
#[derive(Component)]
pub struct BakeryTerminal {
    pub id: u8,
    pub input_buffer: String,
    pub history: Vec<String>,
}
