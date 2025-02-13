use bevy::prelude::*;

use crate::bt_components::{bakery_terminal::BakeryTerminal, sections::Purchasing};

pub fn operate_purchasing(mut query: Query<&mut BakeryTerminal, With<Purchasing>>) {
    if let Ok(terminal) = query.get_single_mut() {}
}
