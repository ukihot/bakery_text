use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::WindowMode;

const FURNACE_NUMBER: usize = 4; // Furnace の数

/// Furnace の一意な ID
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
struct FurnaceId(usize);

/// Furnace のコンポーネント
#[derive(Component)]
struct Furnace {
    input_buffer: String,
    history: Vec<String>,
}

/// フォーカスされている Furnace の ID
#[derive(Resource)]
struct FocusedFurnace(usize);

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
        .insert_resource(FocusedFurnace(0)) // 最初の Furnace にフォーカス
        .add_systems(Startup, install_furnace)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (text_input, display_furnace, switch_furnace))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn install_furnace(mut commands: Commands) {
    // 横方向の間隔と縦方向の間隔
    let horizontal_spacing = 100.0;
    let vertical_spacing = 100.0;

    // 行列形式で配置するために必要な列数を計算
    let num_columns = (FURNACE_NUMBER as f32).sqrt().ceil() as usize;
    let num_rows = (FURNACE_NUMBER as f32 / num_columns as f32).ceil() as usize;

    // 各 Furnace の位置を計算
    for i in 0..FURNACE_NUMBER {
        let row = i / num_columns;
        let col = i % num_columns;

        // 中央に基づいて位置を調整
        let x = (col as f32 - (num_columns as f32 / 2.0 - 0.5)) * horizontal_spacing;
        let y = (row as f32 - (num_rows as f32 / 2.0 - 0.5)) * vertical_spacing;

        // 背景用の四角形を追加
        commands.spawn((
            FurnaceId(i),
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.4),
                custom_size: Some(Vec2::new(400.0, 150.0)), // 背景のサイズ
                ..Default::default()
            },
            Transform::from_xyz(x, y, -1.0),
        ));

        // Furnace テキストを追加
        commands.spawn((
            FurnaceId(i),
            Furnace {
                input_buffer: String::new(),
                history: vec![
                    format!("Furnace Terminal {i}"),
                    "Type 'help' for commands.".to_string(),
                ],
            },
            Text2d::new(format!("Furnace Terminal {i}\nType 'help' for commands.")),
            TextFont {
                font_size: 24.0,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Left),
            TextColor(Color::WHITE),
            Transform::from_xyz(x, y, 0.0), // 背景より Z を大きくする
        ));
    }
}

/// キーボード入力処理 (フォーカスされている Furnace のみを更新)
fn text_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    focused_furnace: Res<FocusedFurnace>,
    mut furnaces: Query<(&FurnaceId, &mut Furnace)>,
) {
    for (id, mut furnace) in furnaces.iter_mut() {
        if id.0 != focused_furnace.0 {
            continue;
        }

        for ev in evr_kbd.read() {
            if ev.state == ButtonState::Released {
                continue;
            }
            match &ev.logical_key {
                Key::Enter => {
                    let input_text = furnace.input_buffer.clone();
                    furnace.history.push(input_text);
                    furnace.input_buffer.clear();
                }
                Key::Backspace => {
                    furnace.input_buffer.pop();
                }
                Key::Character(input) => {
                    if input.chars().any(|c| c.is_control()) {
                        continue;
                    }
                    furnace.input_buffer.push_str(input);
                }
                _ => {}
            }
        }
    }
}

/// Tabキーでフォーカスを切り替える
fn switch_furnace(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut focused_furnace: ResMut<FocusedFurnace>,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Pressed {
            if let Key::Tab = ev.logical_key {
                let prev_id = focused_furnace.0;
                focused_furnace.0 = (focused_furnace.0 + 1) % FURNACE_NUMBER;
                println!(
                    "Switched focus: Furnace {} → Furnace {}",
                    prev_id, focused_furnace.0
                );
            }
        }
    }
}

/// Furnace のテキスト内容を更新
fn display_furnace(mut furnaces: Query<(&Furnace, &mut Text2d)>) {
    for (furnace, mut text) in furnaces.iter_mut() {
        text.0 = furnace.history.join("\n") + &format!("\n> {}", furnace.input_buffer);
    }
}
