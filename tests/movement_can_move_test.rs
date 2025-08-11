use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use tetris_rust_bevy_ver0_16::components::{Piece, Position};
use tetris_rust_bevy_ver0_16::game_color::GameColor;
use tetris_rust_bevy_ver0_16::game_constants::NUM_BLOCKS_Y;
use tetris_rust_bevy_ver0_16::game_types::{GameMap, Presence};
use tetris_rust_bevy_ver0_16::state::GameState;
use tetris_rust_bevy_ver0_16::systems::movement::{can_move, move_piece_down};

#[test]
fn can_move_false_at_bottom() {
    let piece = Piece {
        states: [1632, 1632, 1632, 1632],
        color: GameColor::Yellow,
        current_state: 0,
    };
    let pos = Position {
        x: 0,
        y: (NUM_BLOCKS_Y - 1) as isize,
    };
    let game_map = GameMap::default();
    assert!(!can_move(&piece, &pos, pos.y + 1, &game_map));
}

#[test]
fn move_piece_down_stops_and_writes_on_land() {
    let mut app = App::new();
    app.add_plugins(StatesPlugin);
    app.init_resource::<GameMap>();
    app.insert_state(GameState::Playing);
    app.add_systems(Update, move_piece_down);

    // spawn a simple 1x1-like piece near bottom by using 'O' with position
    let piece = Piece {
        states: [1632, 1632, 1632, 1632],
        color: GameColor::Yellow,
        current_state: 0,
    };
    let pos = Position {
        x: 0,
        y: (NUM_BLOCKS_Y - 2) as isize,
    };
    let entity = app.world_mut().spawn((piece, pos)).id();

    // first update: should land and despawn
    app.update();
    assert!(app.world().get_entity(entity).is_err());

    // ensure something is written on bottom row
    let gm = app.world().resource::<GameMap>();
    let bottom_has_any = gm.0[NUM_BLOCKS_Y - 1]
        .iter()
        .any(|p| matches!(p, Presence::Yes(_)));
    assert!(bottom_has_any);
}
