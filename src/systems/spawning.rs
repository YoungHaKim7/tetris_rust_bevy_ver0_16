use bevy::prelude::*;

use crate::components::{Piece, Position};
use crate::game_constants::NUM_BLOCKS_X;
use crate::game_types::GameMap;
use crate::state::GameState;

pub fn spawn_piece(
    commands: &mut Commands,
    game_map: &GameMap,
    game_state: &mut ResMut<NextState<GameState>>,
) {
    let new_piece = Piece::random();
    let initial_position = Position {
        x: NUM_BLOCKS_X as isize / 2 - 1,
        y: 0,
    };

    if super::movement::can_move(&new_piece, &initial_position, initial_position.y, &game_map) {
        commands.spawn((new_piece, initial_position));
        println!("Spawned new piece");
    } else {
        println!("Game Over! Cannot spawn new piece.");
        game_state.set(GameState::GameOver);
    }
}

pub fn spawn_initial_piece(
    mut commands: Commands,
    game_map: Res<GameMap>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    spawn_piece(&mut commands, &game_map, &mut game_state);
}
