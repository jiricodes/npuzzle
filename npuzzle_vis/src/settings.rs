//! Module to handle various game settings
//! TODO:
//!		- load settings from a file
//!		- save settings to a file
//!
pub mod colorpallete;
use colorpallete::ColorPallete;

pub struct Settings {
	size: f32,
	/// x,y
	tile_scale: (f32, f32),
	/// x, y
	window_margins: (f32, f32),
	font: &'static str,
	colorpallete: ColorPallete,
}

// Default Font Path
pub const FONT_PATH: &str = "fonts/VCR_OSD_MONO.ttf";

impl Default for Settings {
	fn default() -> Self {
		let tile_scale = (0.95, 0.95);
		let window_margins = ((1.0 - tile_scale.0) / 2.0, (1.0 - tile_scale.1) / 2.0);
		Self {
			size: 5.0,
			tile_scale,
			window_margins,
			font: FONT_PATH,
			colorpallete: ColorPallete::default(),
		}
	}
}
