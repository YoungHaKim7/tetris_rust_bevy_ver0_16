use tetris_rust_bevy_ver0_16::game_color::GameColor;
use tetris_rust_bevy_ver0_16::game_types::Presence;
use tetris_rust_bevy_ver0_16::piece_utils::get_block_matrix;

#[test]
fn get_block_matrix_sets_presence_bits() {
    // 'O' piece block (2x2) states use 1632 in this repo
    let matrix = get_block_matrix(1632, GameColor::Yellow);
    // Count Presence::Yes
    let mut count = 0;
    for row in matrix.iter() {
        for cell in row.iter() {
            if matches!(cell, Presence::Yes(_)) {
                count += 1;
            }
        }
    }
    assert!(count >= 4); // at least 2x2
}
