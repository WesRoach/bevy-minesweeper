use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

const WINDOW_HEIGHT: f32 = 400.0;
const WINDOW_WIDTH: f32 = 400.0;

const COLUMN_COUNT: usize = 64;
const ROW_COUNT: usize = 64;
const MINE_COUNT: usize = 16;

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
        .add_systems(Startup, setup_camera)
        .run();
}

// Marker Component tracking the Camera
#[derive(Component)]
struct Camera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
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
fn setup_board(mut Commands: Commands) {
    // Meshes and Materials

    // Create a grid of cells
    for row in 0..ROW_COUNT {
        for column in 0..COLUMN_COUNT {
            Commands.spawn((
                Cell {
                    is_mine: false,
                    state: CellState::Hidden,
                    adjacent_mines: 0,
                },
                Transform::from_xyz(column as f32 * 50.0, row as f32 * 50.0, 0.0),
                GlobalTransform::default(),
            ));
        }
    }
}
