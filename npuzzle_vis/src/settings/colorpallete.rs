//! Game color pallete handler
use crate::settings::FONT_PATH;
use bevy::prelude::*;

#[derive(Component)]
struct CollorPalletePreviewTag;

pub struct ColorPallete {
	background: Color,
	tile_default: Color,
	tile_win: Color,
	font: Color,
}

impl Default for ColorPallete {
	fn default() -> Self {
		Self {
			background: Color::rgba(0.15, 0.15, 0.15, 0.9),
			tile_default: Color::rgb(108.0 / 255.0, 74.0 / 255.0, 74.0 / 255.0),
			tile_win: Color::rgb(111.0 / 255.0, 237.0 / 255.0, 183.0 / 255.0),
			font: Color::rgb(237.0 / 255.0, 237.0 / 255.0, 237.0 / 255.0),
		}
	}
}

impl ColorPallete {
	pub const YELLOW: Self = Self {
		background: Color::rgba(0.15, 0.15, 0.15, 0.9),
		tile_default: Color::rgb(255.0 / 255.0, 184.0 / 255.0, 0.0 / 255.0),
		tile_win: Color::rgb(111.0 / 255.0, 237.0 / 255.0, 183.0 / 255.0),
		font: Color::rgb(237.0 / 255.0, 237.0 / 255.0, 237.0 / 255.0),
	};

	pub fn font(&self) -> Color {
		self.font
	}

	pub fn background(&self) -> Color {
		self.background
	}

	pub fn tile_default(&self) -> Color {
		self.tile_default
	}

	pub fn tile_win(&self) -> Color {
		self.tile_win
	}
}

pub struct ColorPalleteTestPlugin;

/// Camera setup
fn camera_setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn preview(mut cmds: Commands, asset_server: Res<AssetServer>, colorpallete: Res<ColorPallete>) {
	let preview_size: f32 = 200.0;
	let frame_size = Vec2::new(preview_size, preview_size);
	let tile_size = Vec2::new(frame_size.x * 0.5, frame_size.y * 0.5);
	let font_handle: Handle<Font> = asset_server.load(FONT_PATH);
	let text_style = TextStyle {
		font: font_handle,
		font_size: tile_size.y * 0.8,
		color: colorpallete.font(),
	};
	let text_alignment = TextAlignment {
		vertical: VerticalAlign::Center,
		horizontal: HorizontalAlign::Center,
	};
	cmds.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			color: colorpallete.background(),
			custom_size: Some(frame_size),
			..Default::default()
		},
		..Default::default()
	})
	.with_children(|top| {
		top.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: colorpallete.tile_default(),
				custom_size: Some(tile_size),
				..Default::default()
			},
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 1.0),
				..Default::default()
			},
			..Default::default()
		})
		.with_children(|parent| {
			parent.spawn_bundle(Text2dBundle {
				text: Text::with_section("1", text_style.clone(), text_alignment),
				transform: Transform {
					translation: Vec3::new(0.0, 0.0, 2.0),
					..Default::default()
				},
				..Default::default()
			});
		});
	})
	.insert(CollorPalletePreviewTag);
}

impl Plugin for ColorPalleteTestPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ColorPallete::YELLOW)
			.add_startup_system(camera_setup)
			.add_startup_system(preview);
	}
}
