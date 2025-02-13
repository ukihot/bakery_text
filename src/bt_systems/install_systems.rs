use bevy::{color::palettes::css::*, prelude::*};

use crate::bt_components::{
    bakery_terminal::{BakeryTerminal, ModalComponet, OperatorMode, Repository},
    sections::*,
};

pub fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}

pub fn spawn_layout(mut cmd: Commands) {
    cmd.spawn((
        Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
            grid_template_rows: vec![GridTrack::flex(1.), GridTrack::percent(5.)],

            ..default()
        },
        BackgroundColor(Color::srgb_u8(3, 11, 3)),
    ))
    .with_children(|builder| {
        builder
            .spawn((
                Node {
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.414),
                    display: Display::Grid,
                    padding: UiRect::all(Val::Px(16.0)),
                    grid_template_columns: RepeatedGridTrack::flex(4, 1.),
                    grid_template_rows: RepeatedGridTrack::flex(3, 1.),
                    row_gap: Val::Px(10.0),
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            ))
            .with_children(|builder| {
                // Purchasing
                add_section(
                    builder,
                    0,
                    ORANGE,
                    None,
                    OperatorMode::Commander,
                    Purchasing,
                );
                // Pantry
                add_section(
                    builder,
                    1,
                    BISQUE,
                    Some(Repository::new_raw_only()),
                    OperatorMode::default(),
                    Pantry,
                );
                // Mixing
                add_section(
                    builder,
                    2,
                    CRIMSON,
                    Some(Repository::new_raw_with_dough()),
                    OperatorMode::default(),
                    Mixing,
                );
                // Cooling
                add_section(
                    builder,
                    3,
                    AQUA,
                    Some(Repository::new_all()),
                    OperatorMode::default(),
                    Cooling,
                );
                // Shaping
                add_section(
                    builder,
                    4,
                    ORANGE_RED,
                    Some(Repository::new_dough_with_bread()),
                    OperatorMode::default(),
                    Shaping,
                );
                // Baking
                add_section(
                    builder,
                    5,
                    DARK_GREEN,
                    Some(Repository::new_bread_only()),
                    OperatorMode::default(),
                    Baking,
                );
                // Packaging
                add_section(
                    builder,
                    6,
                    FUCHSIA,
                    Some(Repository::new_bread_only()),
                    OperatorMode::default(),
                    Packaging,
                );
                // Quality Control
                add_section(
                    builder,
                    7,
                    TEAL,
                    Some(Repository::new_bread_only()),
                    OperatorMode::default(),
                    QualityControl,
                );
                // Stockroom
                add_section(
                    builder,
                    8,
                    ALICE_BLUE,
                    Some(Repository::new_bread_only()),
                    OperatorMode::default(),
                    Stockroom,
                );
                // Sales Front
                add_section(
                    builder,
                    9,
                    CRIMSON,
                    Some(Repository::new_bread_only()),
                    OperatorMode::default(),
                    SalesFront,
                );
                // Waste Station
                add_section(
                    builder,
                    10,
                    YELLOW_GREEN,
                    None,
                    OperatorMode::default(),
                    WasteStation,
                );
                // Utility
                add_section(builder, 11, SALMON, None, OperatorMode::default(), Utility);
            });

        // Thread bar
        builder.spawn((
            Node {
                grid_column: GridPlacement::span(1),
                ..default()
            },
            BackgroundColor(GREY.into()),
        ));
        // Status bar
        builder.spawn((
            Node {
                grid_column: GridPlacement::span(2),
                ..default()
            },
            BackgroundColor(WHITE.into()),
        ));

        // Modal
        builder.spawn((
            Node {
                position_type: PositionType::Absolute,
                margin: UiRect {
                    top: Val::Auto,
                    bottom: Val::Auto,
                    left: Val::Auto,
                    right: Val::Auto,
                },
                width: Val::Percent(40.),
                height: Val::Percent(20.),
                ..default()
            },
            Text::default(),
            TextColor(Color::BLACK),
            ModalComponet,
            Visibility::Hidden,
            BackgroundColor(Color::WHITE.with_alpha(0.3)),
        ));
    });
}

fn add_section(
    builder: &mut ChildBuilder,
    section_id: u8,
    color: Srgba,
    repository: Option<Repository>,
    operator_mode: OperatorMode,
    section: impl Component + Section,
) {
    let terminal_text = "Purchasing Section Terminal";
    let help_text = "Type 'ls' for cmd.".to_string();

    builder
        .spawn((
            Node {
                display: Display::Grid,
                overflow: Overflow::clip_x(),
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(BLACK.into()),
        ))
        .with_children(|builder| {
            builder.spawn((
                Node::DEFAULT,
                BackgroundColor(color.into()),
                Text::default(),
                TextFont {
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new(JustifyText::Left, LineBreak::AnyCharacter),
                BakeryTerminal {
                    id: section_id,
                    input_buffer: String::new(),
                    history: vec![terminal_text.to_string(), help_text.clone()],
                    ..Default::default()
                },
                operator_mode,
                section,
            ));
            if let Some(repo) = repository {
                builder.spawn(repo);
            }
        });
}
