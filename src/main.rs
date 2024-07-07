use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::{PresentMode, PrimaryWindow, WindowTheme},
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
const MINE_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

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
        .add_event::<MouseButtonInput>()
        .add_systems(Update, mouse_button_events)
        .run();
}

// Marker Component tracking the Camera
#[derive(Component)]
struct MainCamera;

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
        MainCamera,
    ));
}

// The board is a grid of cells
// Each cell is a square
#[derive(Component, Debug)]
struct Cell {
    is_mine: bool,
    state: CellState,
    adjacent_mines: u8, // Number of adjacent mines
    position: Position,
}

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
enum CellState {
    Hidden,
    Flagged,
    Revealed(Revealed),
}

#[derive(Component, Debug)]
enum Revealed {
    Mine,
    AdjacentMines(u8),
}

#[derive(Component)]
struct Mine;

// Initialize Minesweeper board
fn setup_board(mut commands: Commands) {
    // Create vector of mines (bool)
    let mut mine_positions: Vec<bool> = vec![false; ROW_COUNT * COLUMN_COUNT];

    let mut mines: usize = 0;
    while mines < MINE_COUNT {
        let mine_position = rand::random::<usize>() % (ROW_COUNT * COLUMN_COUNT);
        if !mine_positions[mine_position] {
            mine_positions[mine_position] = true;
            mines += 1;
            println!("Mine position: {}", mine_position);
        }
    }

    // Create a grid of cells
    let mut cell: usize = 0;
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
                    is_mine: mine_positions[cell].clone(),
                    state: CellState::Hidden,
                    adjacent_mines: 0,
                    position: Position {
                        x: x_position,
                        y: y_position,
                    },
                },
            ));

            if mine_positions[cell] {
                println!("Created Mine @ ({}, {})", x_position, y_position);
            }

            // Iterate current cell index
            cell += 1;
        }
    }
}

// Emit a mouse left and/or right click event and capture the position of the mouse
fn mouse_button_events(
    mut event_mouse_button: EventReader<MouseButtonInput>, // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // query to get the cells
    q_cells: Query<&Cell>,
) {
    for ev in event_mouse_button.read() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            println!("World coords: {}/{}", world_position.x, world_position.y);
            // check if the cursor is inside a cell
            for cell in q_cells.iter() {
                if world_position.x >= cell.position.x
                    && world_position.x <= cell.position.x + CELL_SIZE
                    && world_position.y >= cell.position.y
                    && world_position.y <= cell.position.y + CELL_SIZE
                {
                    println!("Cell clicked: {:?}", cell.position);
                }
            }
        }
    }
}
