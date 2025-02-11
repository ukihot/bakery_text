mod bt_components;
mod bt_resources;
mod bt_systems;

use bevy::{prelude::*, window::WindowMode};

use crate::{
    bt_components::{bakery_terminal::*, dialog::*},
    bt_resources::{bakery_terminal::*, dialog::*, plant::*},
    bt_systems::{bakery_terminal::*, dialog::*, emit_commands::*},
};

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
        .add_event::<Emitation>()
        .insert_resource(FocusedSection(0)) // Initially focus on the first BakeryTerminal
        .insert_resource(ExitDialog { active: false })
        .insert_resource(PurchasingSection::default())
        .insert_resource(PantrySection::default())
        .insert_resource(MixingSection::default())
        .insert_resource(CoolingSection::default())
        .insert_resource(ShapingSection::default())
        .insert_resource(BakingSection::default())
        .insert_resource(PackagingSection::default())
        .insert_resource(QualitySection::default())
        .insert_resource(StockroomSection::default())
        .insert_resource(SalesSection::default())
        .insert_resource(WasteSection::default())
        .insert_resource(UtilitySection::default())
        .add_systems(Startup, (setup_camera, install_bakery))
        .add_systems(Update, (text_input, display, activate_section))
        .add_systems(
            PostUpdate,
            (handle_exit_dialog, handle_esc_key, handle_commands),
        )
        .run();
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}
