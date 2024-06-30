use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

const WINDOW_HEIGHT: f32 = 400.0;
const WINDOW_WIDTH: f32 = 400.0;

const COLUMN_COUNT: usize = 20;
const ROW_COUNT: usize = 20;
const GAP_BETWEEN_CELLS: f32 = 2.0;
const CELL_SIZE: f32 =
    (WINDOW_HEIGHT - (GAP_BETWEEN_CELLS * (ROW_COUNT as f32))) / ROW_COUNT as f32;
// const CELL_SIZE: f32 = (WINDOW_HEIGHT / ROW_COUNT as f32);

const MINE_COUNT: usize = 10;
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
            // LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_board))
        .run();
}

// Marker Component tracking the Camera
#[derive(Component)]
struct Camera;

fn setup_camera(mut commands: Commands) {
    // Center Camera on the center of the mines
    // All mines should be visible in the camera
    let x = ((COLUMN_COUNT as f32 * (WINDOW_WIDTH / COLUMN_COUNT as f32)) / 2.0) - CELL_SIZE / 2.0;
    let y = ((ROW_COUNT as f32 * (WINDOW_HEIGHT / ROW_COUNT as f32)) / 2.0) - CELL_SIZE / 2.0;
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
    position: Position,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
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
            let x_position = column as f32 * (CELL_SIZE + GAP_BETWEEN_CELLS);
            let y_position = row as f32 * (CELL_SIZE + GAP_BETWEEN_CELLS);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: MINE_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x_position, y_position, 0.0),
                        scale: Vec3::splat(CELL_SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Cell {
                    is_mine: false,
                    state: CellState::Hidden,
                    adjacent_mines: 0,
                    position: Position {
                        x: x_position,
                        y: y_position,
                    },
                },
            ));
        }
    }
}
