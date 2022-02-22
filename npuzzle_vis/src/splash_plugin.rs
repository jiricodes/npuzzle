use crate::utils::despawn_screen;
use bevy::prelude::*;

pub struct SplashPlugin;

#[derive(Component)]
struct SplashScreenTag;

struct SplashTimer(Timer);
