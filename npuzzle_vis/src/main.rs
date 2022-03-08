use bevy::prelude::*;

mod puzzle_plugin;
use puzzle_plugin::Npuzzle;
pub mod components;
mod settings;
use settings::colorpallete::ColorPalleteTestPlugin;
pub mod utils;

// Animaitons
const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(ColorPalleteTestPlugin)
		.run();
}
