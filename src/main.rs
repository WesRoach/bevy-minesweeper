use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

const WINDOW_HEIGHT: f32 = 400.0;
const WINDOW_WIDTH: f32 = 400.0;

const COLUMN_COUNT: usize = 20;
const ROW_COUNT: usize = 20;
const MINE_COUNT: usize = 10;

// const GAP_BETWEEN_CELLS: f32 = 2.0;

const MINE_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Minesweeper".into(),
                    name: Some("bevy.app".into()),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R, etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_board))
        .run();
}

// Marker Component tracking the Camera
#[derive(Component)]
struct Camera;

fn setup_camera(mut commands: Commands) {
    // Determine Camera focus/offset
    let x = (WINDOW_WIDTH / 2.0) - (WINDOW_WIDTH / COLUMN_COUNT as f32 / 2.0);
    let y = (WINDOW_HEIGHT / 2.0) - (WINDOW_HEIGHT / ROW_COUNT as f32 / 2.0);
    let z = 1.0;

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(x, y, z),
            ..Default::default()
        },
        Camera,
    ));
}

// The board is a grid of cells
// Each cell is a square
#[derive(Component)]
struct Cell {
    is_mine: bool,
    state: CellState,
    adjacent_mines: u8, // Number of adjacent mines
}

enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

// Initialize Minesweeper board
fn setup_board(mut commands: Commands) {
    // Meshes and Materials

    // Create a grid of cells
    for row in 0..ROW_COUNT {
        for column in 0..COLUMN_COUNT {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: MINE_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            (column as f32 * (WINDOW_WIDTH / COLUMN_COUNT as f32)),
                            // + (GAP_BETWEEN_CELLS * column as f32),
                            (row as f32 * (WINDOW_HEIGHT / ROW_COUNT as f32)),
                            // + (GAP_BETWEEN_CELLS * row as f32),
                            0.0,
                        ),
                        scale: Vec3::splat(WINDOW_WIDTH / COLUMN_COUNT as f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Cell {
                    is_mine: false,
                    state: CellState::Hidden,
                    adjacent_mines: 0,
                },
            ));
        }
    }
}
