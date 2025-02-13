mod bt_components;
mod bt_resources;
mod bt_systems;

use bevy::{prelude::*, window::WindowMode};

use crate::{
    bt_components::bakery_terminal::*,
    bt_resources::forcused_section::*,
    bt_systems::{
        emit_commands::*,
        handle_inputs::*,
        install_systems::*,
        operate_purchasing::*,
        power_systems::*,
    },
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    #[default]
    Running,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .init_state::<PausedState>()
        .add_event::<Emitation>()
        .insert_resource(FocusedSection(0))
        .add_systems(Startup, (setup_camera, spawn_layout))
        .add_systems(
            Update,
            (
                handle_text_input,
                handle_commands,
                display,
                handle_esc_key,
                switch_section,
                operate_purchasing,
            )
                .run_if(in_state(PausedState::Running)),
        )
        .add_systems(Update, ask_exit.run_if(in_state(PausedState::Paused)))
        .run();
}
