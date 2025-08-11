use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct Level {
    pub value: u32,
    pub lines_cleared_in_level: u32,
}

pub use Level as GameLevel;
pub use Score as GameScore;
