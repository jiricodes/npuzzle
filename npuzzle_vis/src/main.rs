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
struct Tile {
	pub value: usize,
}

#[derive(Component)]
struct Value(usize);

type MoveQueue = VecDeque<Direction>;

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
			.add_startup_system(camera_setup)
			.add_startup_system(setup)
			.add_system(keyboard_input);
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

fn make_move(mut grid: ResMut<Grid2D>, mut move_queue: ResMut<MoveQueue>, query: Query<&mut Tile>) {
	if !move_queue.is_empty() {
		let current_move = move_queue.pop_front().unwrap();
		let old = grid.get_zero();
		if grid.do_move(&current_move).is_ok() {
			let new = grid.get_zero();
			let val = grid.get_value(old).unwrap();
			// now visu - put zero tile to new and tile with value in old
			for Tiel
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
	let start_x = -WIN_SIZE / 2.0 + MARGIN + TILE_SIZE / 2.0;
	let start_y = WIN_SIZE / 2.0 + MARGIN + TILE_SIZE / 2.0;
	for row in 0..SIZE as usize {
		for col in 0..SIZE as usize {
			let index = row * SIZE as usize + col;
			let value = grid.get_value(Coords(row as i32, col as i32)).unwrap();
			let label = format!("{}", value);
			println!("[{}]: {}", index, label);
			commands
				.spawn_bundle(SpriteBundle {
					transform: Transform {
						translation: Vec3::new(
							start_x + (TILE_SIZE + MARGIN) * col as f32,
							start_y - (TILE_SIZE + MARGIN) * row as f32,
							0.0,
						),
						..Default::default()
					},
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
				.insert(Tile { value });
		}
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(Npuzzle)
		.run();
}
