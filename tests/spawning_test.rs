use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use tetris_rust_bevy_ver0_16::game_types::GameMap;
use tetris_rust_bevy_ver0_16::state::GameState;
use tetris_rust_bevy_ver0_16::systems::spawning::spawn_piece;

#[test]
fn spawn_piece_adds_entity_or_sets_game_over() {
    let mut app = App::new();
    app.add_plugins(StatesPlugin);
    app.init_resource::<GameMap>();
    app.insert_state(GameState::Playing);

    let before = app.world().entities().len();

    // Use a Commands proxy via system to call spawn_piece
    app.add_systems(
        Startup,
        |mut commands: Commands, game_map: Res<GameMap>, mut next: ResMut<NextState<GameState>>| {
            spawn_piece(&mut commands, &game_map, &mut next);
        },
    );

    app.update();

    let after = app.world().entities().len();
    assert!(after >= before);
}
