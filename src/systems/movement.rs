use bevy::prelude::*;

use crate::components::{Piece, Position};
use crate::game_constants::{NUM_BLOCKS_X, NUM_BLOCKS_Y};
use crate::game_types::{GameMap, Presence};
use crate::piece_utils::get_block_matrix;
use crate::state::GameState;

use super::spawning::spawn_piece;

pub fn move_piece_down(
    mut commands: Commands,
    mut query_piece: Query<(Entity, &mut Piece, &mut Position)>,
    mut game_map: ResMut<GameMap>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, piece, mut position)) = query_piece.single_mut() {
        let new_y = position.y + 1;
        if can_move(&piece, &position, new_y, &game_map) {
            position.y = new_y;
            println!("Piece moved down to y: {}", position.y);
        } else {
            let piece_matrix = get_block_matrix(piece.states[piece.current_state], piece.color);
            for my in 0..4 {
                for mx in 0..4 {
                    if let Presence::Yes(color) = piece_matrix[my][mx] {
                        let map_x = position.x + mx as isize;
                        let map_y = position.y + my as isize;
                        if map_x >= 0
                            && map_x < NUM_BLOCKS_X as isize
                            && map_y >= 0
                            && map_y < NUM_BLOCKS_Y as isize
                        {
                            game_map.0[map_y as usize][map_x as usize] = Presence::Yes(color);
                        }
                    }
                }
            }
            commands.entity(entity).despawn();
            spawn_piece(&mut commands, &game_map, &mut game_state);
            println!("Piece landed at y: {}", position.y);
            println!("Piece finalized and added to game map.");
        }
    }
}

pub fn can_move(piece: &Piece, current_pos: &Position, new_y: isize, game_map: &GameMap) -> bool {
    let piece_matrix = get_block_matrix(piece.states[piece.current_state], piece.color);
    for my in 0..4 {
        for mx in 0..4 {
            if let Presence::Yes(_) = piece_matrix[my][mx] {
                let block_x = current_pos.x + mx as isize;
                let block_y = new_y + my as isize;

                if block_y >= NUM_BLOCKS_Y as isize {
                    return false;
                }

                if block_x >= 0 && block_x < NUM_BLOCKS_X as isize && block_y >= 0 {
                    if let Presence::Yes(_) = game_map.0[block_y as usize][block_x as usize] {
                        return false;
                    }
                }
            }
        }
    }
    true
}
