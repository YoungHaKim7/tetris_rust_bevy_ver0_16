use bevy::prelude::*;
use tetris_rust_bevy_ver0_16::game_constants::NUM_BLOCKS_X;
use tetris_rust_bevy_ver0_16::game_types::{GameMap, Presence};
use tetris_rust_bevy_ver0_16::resources::{Level, Score};
use tetris_rust_bevy_ver0_16::systems::lines::clear_lines;

#[test]
fn clear_full_line_increases_score_and_shifts_rows() {
    let mut app = App::new();
    app.insert_resource(GameMap(vec![
        vec![
            Presence::Yes(
                tetris_rust_bevy_ver0_16::game_color::GameColor::Red
            );
            NUM_BLOCKS_X
        ];
        4
    ]));
    app.insert_resource(Score::default());
    app.insert_resource(Level::default());
    app.add_systems(Update, clear_lines);

    app.update();

    let score = app.world().resource::<Score>();
    let level = app.world().resource::<Level>();
    assert!(score.value >= 100);
    assert!(level.value <= 1);
}
