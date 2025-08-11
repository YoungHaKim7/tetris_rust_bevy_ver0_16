use bevy::prelude::*;
use std::time::Duration;

use crate::game_constants::{LEVEL_TIMES, NUM_LEVELS};
use crate::resources::Level;

pub fn update_gravity_speed(level: Res<Level>, mut fixed_time: ResMut<Time<Fixed>>) {
    if level.is_changed() {
        let level_index = level.value as usize;
        if level_index < NUM_LEVELS {
            let new_speed_ms = LEVEL_TIMES[level_index];
            let new_speed_secs = new_speed_ms as f32 / 1000.0;
            fixed_time.set_wrap_period(Duration::from_secs_f32(new_speed_secs));
            println!("Gravity speed updated to: {}s", new_speed_secs);
        }
    }
}
