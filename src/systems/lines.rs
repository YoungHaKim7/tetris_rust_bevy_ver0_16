use bevy::prelude::*;

use crate::game_constants::NUM_BLOCKS_X;
use crate::game_types::{GameMap, Presence};
use crate::resources::Level;
use crate::resources::Score;

pub fn clear_lines(mut game_map: ResMut<GameMap>, mut score: ResMut<Score>, mut level: ResMut<Level>) {
    let mut lines_cleared = 0;
    let mut rows_to_clear = Vec::new();

    for y in 0..game_map.0.len() {
        let mut is_full = true;
        for x in 0..NUM_BLOCKS_X {
            if let Presence::No = game_map.0[y][x] {
                is_full = false;
                break;
            }
        }
        if is_full {
            rows_to_clear.push(y);
        }
    }

    for &row_to_clear in rows_to_clear.iter().rev() {
        lines_cleared += 1;
        game_map.0.remove(row_to_clear);
        game_map.0.insert(0, vec![Presence::No; NUM_BLOCKS_X]);
    }

    if lines_cleared > 0 {
        score.value += lines_cleared as u32 * 100;
        level.lines_cleared_in_level += lines_cleared as u32;
        if level.lines_cleared_in_level >= 10 {
            level.value += 1;
            level.lines_cleared_in_level = 0;
        }
        println!(
            "Cleared {} lines! Current score: {}",
            lines_cleared, score.value
        );
    }
}
