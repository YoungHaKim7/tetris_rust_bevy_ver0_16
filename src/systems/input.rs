use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

use crate::components::{Piece, Position};
use crate::game_constants::{NUM_BLOCKS_X, NUM_BLOCKS_Y};
use crate::game_types::{GameMap, Presence};
use crate::piece_utils::get_block_matrix;
use crate::state::GameState;

use super::movement::can_move;
use super::spawning::spawn_piece;

pub fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut Position, &mut Piece)>,
    mut game_map: ResMut<GameMap>,
    mut score: ResMut<crate::resources::Score>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, mut position, mut piece)) = query.single_mut() {
        if keyboard_input.just_pressed(bevy::input::keyboard::KeyCode::ArrowLeft) {
            let new_x = position.x - 1;
            if can_move_horizontally(&piece, &position, new_x, &game_map) {
                position.x = new_x;
            }
        }
        if keyboard_input.just_pressed(bevy::input::keyboard::KeyCode::ArrowRight) {
            let new_x = position.x + 1;
            if can_move_horizontally(&piece, &position, new_x, &game_map) {
                position.x = new_x;
            }
        }
        if keyboard_input.just_pressed(bevy::input::keyboard::KeyCode::ArrowDown) {
            let new_y = position.y + 1;
            if can_move(&piece, &position, new_y, &game_map) {
                position.y = new_y;
            }
        }

        if keyboard_input.just_pressed(bevy::input::keyboard::KeyCode::Space) {
            println!("Space key pressed");
            let mut final_y = position.y;
            while can_move(&piece, &position, final_y + 1, &game_map) {
                final_y += 1;
            }

            if final_y > position.y {
                score.value += (final_y - position.y) as u32;
                position.y = final_y;
            }

            let piece_matrix = get_block_matrix(piece.states[piece.current_state], piece.color);
            for (my, row) in piece_matrix.iter().enumerate() {
                for (mx, cell) in row.iter().enumerate() {
                    if let Presence::Yes(color) = *cell {
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
        }

        if keyboard_input.just_pressed(bevy::input::keyboard::KeyCode::ArrowUp) {
            let old_state = piece.current_state;
            let next_state = (piece.current_state + 1) % 4;
            let mut rotated_piece = *piece;
            rotated_piece.current_state = next_state;

            if can_rotate(&rotated_piece, &position, &game_map) {
                piece.current_state = next_state;
            } else {
                piece.current_state = old_state;
            }
        }
    }
}

pub fn can_rotate(piece: &Piece, current_pos: &Position, game_map: &GameMap) -> bool {
    let piece_matrix = get_block_matrix(piece.states[piece.current_state], piece.color);
    for (my, row) in piece_matrix.iter().enumerate() {
        for (mx, cell) in row.iter().enumerate() {
            if let Presence::Yes(_) = *cell {
                let block_x = current_pos.x + mx as isize;
                let block_y = current_pos.y + my as isize;

                if block_x < 0
                    || block_x >= NUM_BLOCKS_X as isize
                    || block_y < 0
                    || block_y >= NUM_BLOCKS_Y as isize
                {
                    return false;
                }

                if let Presence::Yes(_) = game_map.0[block_y as usize][block_x as usize] {
                    return false;
                }
            }
        }
    }
    true
}

pub fn can_move_horizontally(
    piece: &Piece,
    current_pos: &Position,
    new_x: isize,
    game_map: &GameMap,
) -> bool {
    let piece_matrix = get_block_matrix(piece.states[piece.current_state], piece.color);
    for (my, row) in piece_matrix.iter().enumerate() {
        for (mx, cell) in row.iter().enumerate() {
            if let Presence::Yes(_) = *cell {
                let block_x = new_x + mx as isize;
                let block_y = current_pos.y + my as isize;

                if block_x < 0 || block_x >= NUM_BLOCKS_X as isize {
                    return false;
                }

                if block_y >= 0
                    && block_y < NUM_BLOCKS_Y as isize
                    && block_x >= 0
                    && block_x < NUM_BLOCKS_X as isize
                {
                    if let Presence::Yes(_) = game_map.0[block_y as usize][block_x as usize] {
                        return false;
                    }
                }
            }
        }
    }
    true
}
