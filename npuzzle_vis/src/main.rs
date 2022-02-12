use bevy::prelude::*;

pub struct Npuzzle;

// Tiles
const TILE_SIZE: f32 = 100.0;
const TILE_COLOR: Color = Color::rgb(0.75, 0.5, 0.5);

// Font
const FONT_PATH: &str = "fonts/VCR_OSD_MONO.ttf";
const FONT_SIZE: f32 = 0.5 * TILE_SIZE;
const FONT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

// Puzzle
const SIZE: f32 = 5.0;
const BACKGROUND: Color = Color::rgba(0.15, 0.15, 0.15, 0.9);
const MARGIN: f32 = TILE_SIZE * 0.05;
const WIN_SIZE: f32 = MARGIN * (SIZE + 1.0) + TILE_SIZE * SIZE;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Value(usize);

impl Plugin for Npuzzle {
	fn build(&self, app: &mut App) {
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
			.add_startup_system(camera_setup)
			.add_startup_system(setup);
	}
}

fn camera_setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
	let start_y = -WIN_SIZE / 2.0 + MARGIN + TILE_SIZE / 2.0;
	for row in 0..SIZE as usize {
		for col in 0..SIZE as usize {
			let value = row * SIZE as usize + col;
			let label = format!("{}", value);
			commands
				.spawn_bundle(SpriteBundle {
					transform: Transform {
						translation: Vec3::new(
							start_x + (TILE_SIZE + MARGIN) * col as f32,
							start_y + (TILE_SIZE + MARGIN) * row as f32,
							0.0,
						),
						..Default::default()
					},
					sprite: Sprite {
						color: TILE_COLOR,
						custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
						..Default::default()
					},
					..Default::default()
				})
				.with_children(|parent| {
					parent.spawn_bundle(Text2dBundle {
						text: Text::with_section(label, text_style.clone(), text_alignment),
						..Default::default()
					});
				})
				.insert(Tile)
				.insert(Value(value));
		}
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(Npuzzle)
		.run();
}
