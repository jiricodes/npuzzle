use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;
use npuzzle_lib::generator::Generator;
use npuzzle_lib::generator::PuzzleType;
use npuzzle_lib::grid2d::{Coords, Direction, Grid2D};
use npuzzle_lib::grid_traits::Grid;
use std::collections::VecDeque;

use crate::components::{Tile, TilePosition, TileSize, Value};

type MoveQueue = VecDeque<Direction>;

pub struct Npuzzle;

pub struct Game {
    grid: Grid2D,
    solution: Grid2D,
    iter: usize,
    moves: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            grid: Grid2D::with_capacity(SIZE as usize, SIZE as usize),
            solution: Grid2D::new(),
            iter: 100, // change to random?
            moves: 0,
        }
    }
}

// Tiles
const TILE_SIZE: f32 = 100.0;
const TILE_COLOR: Color = Color::rgb(108.0 / 255.0, 74.0 / 255.0, 74.0 / 255.0);
const TILE_END_COLOR: Color = Color::rgb(111.0 / 255.0, 237.0 / 255.0, 183.0 / 255.0);

// Font
const FONT_PATH: &str = "fonts/VCR_OSD_MONO.ttf";
const FONT_SIZE: f32 = 60.0;
const FONT_COLOR: Color = Color::rgb(237.0 / 255.0, 237.0 / 255.0, 237.0 / 255.0);

// Puzzle
const SIZE: f32 = 5.0;
const BACKGROUND: Color = Color::rgba(0.15, 0.15, 0.15, 0.9);
const MARGIN: f32 = TILE_SIZE * 0.05;
const WIN_SIZE: f32 = MARGIN * (SIZE + 1.0) + TILE_SIZE * SIZE;

// Game Settings
const PUZZLETYPE: PuzzleType = PuzzleType::LinesNN;
const ITERS: usize = 100;

impl Plugin for Npuzzle {
    fn build(&self, app: &mut App) {
        let grid = Grid2D::with_capacity(SIZE as usize, SIZE as usize);
        let mut gen = Generator::new(grid, PUZZLETYPE);
        if gen.generate_solution().is_err() {
            panic!("Gen solution failed");
        }
        let solution = gen.get_grid();
        if gen.generate_random(ITERS).is_err() {
            panic!("Gen random failed");
        }
        let grid = gen.get_grid();
        let game = Game {
            grid,
            solution,
            ..Default::default()
        };
        let move_queue: MoveQueue = MoveQueue::new();
        app.insert_resource(ClearColor(BACKGROUND))
            .insert_resource(WindowDescriptor {
                width: WIN_SIZE,
                height: WIN_SIZE,
                title: "N-Puzzle".to_string(),
                transparent: true,
                decorations: false,
                ..Default::default()
            })
            .insert_resource(game)
            .insert_resource(move_queue)
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(tile_size_system)
                    .with_system(tile_position_system)
                    .with_system(game_end_check),
            )
            .add_startup_system(camera_setup)
            .add_startup_system(setup)
            .add_system(keyboard_input)
            .add_system(make_move);
    }
}

/// System to handle resizing tiles based on window size
fn tile_size_system(win: Res<Windows>, mut q: Query<(&TileSize, &mut Sprite), With<Tile>>) {
    let w = win.get_primary().unwrap().width() as f32;
    let h = win.get_primary().unwrap().height() as f32;
    for (size, mut sprite) in q.iter_mut() {
        let sx = (w / SIZE) * size.x;
        let sy = (h / SIZE) * size.y;
        sprite.custom_size = Some(Vec2::new(sx, sy));
    }
}

/// System to handle tiles positioning based - translates grid based to window based locations
fn tile_position_system(
    win: Res<Windows>,
    mut q: Query<(&mut Transform, &TilePosition), With<Tile>>,
) {
    let w = win.get_primary().unwrap().width() as f32;
    let h = win.get_primary().unwrap().height() as f32;
    for (mut tx, pos) in q.iter_mut() {
        let x = (pos.col as f32 / SIZE) * w - w / 2.0 + (w / SIZE) / 2.0;
        let y = -1.0 * (pos.row as f32 / SIZE) * h + h / 2.0 - (h / SIZE) / 2.0;
        tx.translation = Vec3::new(x, y, 0.0);
    }
}

/// Should be perhaps split into two once game state is introduced
fn game_end_check(game: Res<Game>, mut query: Query<&mut Sprite, With<Tile>>) {
    let mut color = TILE_COLOR;
    if game.grid == game.solution {
        color = TILE_END_COLOR;
        println!("Puzzle solved after {} moves.", game.moves);
    }
    for mut s in query.iter_mut() {
        s.color = color;
    }
}
/// Keyboard input handler
fn keyboard_input(mut key_events: EventReader<KeyboardInput>, mut move_queue: ResMut<MoveQueue>) {
    for ev in key_events.iter() {
        match ev.state {
            ElementState::Released => match ev.key_code {
                Some(kc) => match kc {
                    KeyCode::Up => {
                        move_queue.push_back(Direction::Up);
                    }
                    KeyCode::Down => {
                        move_queue.push_back(Direction::Down);
                    }
                    KeyCode::Right => {
                        move_queue.push_back(Direction::Right);
                    }
                    KeyCode::Left => {
                        move_queue.push_back(Direction::Left);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}

/// Tile movement handler
fn make_move(
    mut game: ResMut<Game>,
    mut move_queue: ResMut<MoveQueue>,
    mut query: Query<(&Value, &mut TilePosition), With<Tile>>,
) {
    if !move_queue.is_empty() {
        let current_move = move_queue.pop_front().unwrap();
        let old = game.grid.get_zero();
        if game.grid.do_move(&current_move).is_ok() {
            let new = game.grid.get_zero();
            let val = game.grid.get_value(old).unwrap();
            // now visu - put zero tile to new and tile with value in old
            for (v, mut pos) in query.iter_mut() {
                if v.0 == val {
                    pos.row = old.0 as usize;
                    pos.col = old.1 as usize;
                }
                if v.0 == 0 {
                    pos.row = new.0 as usize;
                    pos.col = new.1 as usize;
                }
            }
        }
        game.moves += 1;
    }
}

/// Camera setup
fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

/// Game setup handler
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game: Res<Game>) {
    let font_handle: Handle<Font> = asset_server.load(FONT_PATH);
    let text_style = TextStyle {
        font: font_handle,
        font_size: FONT_SIZE,
        color: FONT_COLOR,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    for row in 0..SIZE as usize {
        for col in 0..SIZE as usize {
            let value = game.grid.get_value(Coords(row as i32, col as i32)).unwrap();
            let label = format!("{}", value);
            println!("[{}, {}]: {}", row, col, label);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: TILE_COLOR,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    visibility: Visibility {
                        is_visible: value != 0,
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::with_section(label, text_style.clone(), text_alignment),
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 1.0),
                            ..Default::default()
                        },
                        visibility: Visibility {
                            is_visible: value != 0,
                        },
                        ..Default::default()
                    });
                })
                .insert(Tile)
                .insert(Value(value))
                .insert(TileSize { x: 0.95, y: 0.95 })
                .insert(TilePosition { row, col });
        }
    }
}
