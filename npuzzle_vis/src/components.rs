use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component)]
pub struct Tile;

#[derive(Component, PartialEq, Clone, Copy, Debug, Inspectable)]
pub struct TilePosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Component, Inspectable)]
pub struct TileSize {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Inspectable)]
pub struct Value(pub usize);
