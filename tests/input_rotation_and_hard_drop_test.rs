use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use tetris_rust_bevy_ver0_16::components::{Piece, Position};
use tetris_rust_bevy_ver0_16::game_types::GameMap;
use tetris_rust_bevy_ver0_16::resources::Score;
use tetris_rust_bevy_ver0_16::state::GameState;
use tetris_rust_bevy_ver0_16::systems::input::{can_rotate, handle_input};

#[test]
fn rotate_block_within_bounds() {
    let game_map = GameMap::default();
    let pos = Position { x: 5, y: 5 };
    let mut piece = Piece::default();
    piece.states = [17984, 3648, 19520, 19968];

    assert!(can_rotate(&piece, &pos, &game_map));
}

#[test]
fn hard_drop_increases_score() {
    let mut app = App::new();
    app.add_plugins(StatesPlugin);
    app.init_resource::<GameMap>();
    app.insert_resource(Score::default());
    app.insert_state(GameState::Playing);

    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.add_systems(Update, handle_input);

    // spawn a small piece somewhere near top
    let piece = Piece {
        states: [1632, 1632, 1632, 1632],
        ..Default::default()
    };
    let pos = Position { x: 0, y: 0 };
    app.world_mut().spawn((piece, pos));

    // press Space to trigger hard drop
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::Space);
    }

    app.update();

    // score should have increased
    let score = app.world().resource::<Score>();
    assert!(score.value > 0);
}
