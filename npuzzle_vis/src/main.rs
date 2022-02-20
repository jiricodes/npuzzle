use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;
use npuzzle_lib::generator::PuzzleType;
use npuzzle_lib::grid2d::{Coords, Direction, Grid2D};
use npuzzle_lib::grid_traits::Grid;
use std::collections::VecDeque;

pub struct Npuzzle;

// Tiles
const TILE_SIZE: f32 = 100.0;
const TILE_COLOR: Color = Color::rgb(0.75, 0.5, 0.5);

// Font
const FONT_PATH: &str = "fonts/VCR_OSD_MONO.ttf";
const FONT_SIZE: f32 = 60.0;
const FONT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

// Puzzle
const SIZE: f32 = 5.0;
const BACKGROUND: Color = Color::rgba(0.15, 0.15, 0.15, 0.9);
const MARGIN: f32 = TILE_SIZE * 0.05;
const WIN_SIZE: f32 = MARGIN * (SIZE + 1.0) + TILE_SIZE * SIZE;

// Animaitons
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Tile;

#[derive(Component, PartialEq, Clone, Copy, Debug)]
struct TilePosition {
	row: usize,
	col: usize,
}

#[derive(Component)]
struct TileSize {
	x: f32,
	y: f32,
}

#[derive(Component)]
struct Value(usize);

type MoveQueue = VecDeque<Direction>;

fn tile_size_system(win: Res<Windows>, mut q: Query<(&TileSize, &mut Sprite), With<Tile>>) {
	let w = win.get_primary().unwrap().width() as f32;
	let h = win.get_primary().unwrap().height() as f32;
	for (size, mut sprite) in q.iter_mut() {
		let sx = (w / SIZE) * size.x;
		let sy = (h / SIZE) * size.y;
		sprite.custom_size = Some(Vec2::new(sx, sy));
	}
}

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

impl Plugin for Npuzzle {
	fn build(&self, app: &mut App) {
		let mut grid = Grid2D::new();
		grid.from_2dvec(PuzzleType::LinesNN.get_template(SIZE as usize, SIZE as usize))
			.unwrap();
		let move_queue: MoveQueue = MoveQueue::new();
		app.insert_resource(ClearColor(BACKGROUND))
			.insert_resource(WindowDescriptor {
				width: WIN_SIZE,
				height: WIN_SIZE,
				title: "N-Puzzle".to_string(),
				resizable: false, // Change this later
				transparent: true,
				decorations: false,
				..Default::default()
			})
			.insert_resource(grid)
			.insert_resource(move_queue)
			.add_system_set_to_stage(
				CoreStage::PostUpdate,
				SystemSet::new()
					.with_system(tile_size_system)
					.with_system(tile_position_system),
			)
			.add_startup_system(camera_setup)
			.add_startup_system(setup)
			.add_system(keyboard_input)
			.add_system(make_move);
	}
}

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

fn make_move(
	mut grid: ResMut<Grid2D>,
	mut move_queue: ResMut<MoveQueue>,
	mut query: Query<(&Value, &mut TilePosition), With<Tile>>,
) {
	if !move_queue.is_empty() {
		let current_move = move_queue.pop_front().unwrap();
		let old = grid.get_zero();
		if grid.do_move(&current_move).is_ok() {
			let new = grid.get_zero();
			let val = grid.get_value(old).unwrap();
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
	}
}

fn camera_setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, grid: Res<Grid2D>) {
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
			let value = grid.get_value(Coords(row as i32, col as i32)).unwrap();
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
						is_visible: true, //value != 0,
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

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(Npuzzle)
		.run();
}
