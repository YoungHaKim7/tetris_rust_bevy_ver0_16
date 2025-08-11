use bevy::prelude::*;

mod components;
mod game_color;
mod game_constants;
mod game_types;
mod piece_utils;
mod resources;
mod state;
mod systems;

use game_color::GameColor;
use game_constants::{HEIGHT, TITLE, WIDTH};
use game_types::GameMap;
use state::GameState;

use systems::input::handle_input;
use systems::lines::clear_lines;
use systems::movement::move_piece_down;
use systems::rendering::draw_blocks;
use systems::setup::setup_camera;
use systems::spawning::spawn_initial_piece;
use systems::time::update_gravity_speed;

fn main() {
    App::new()
        .insert_resource(ClearColor(GameColor::Gray.into()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIDTH as f32, HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameMap>()
        .init_resource::<resources::Score>()
        .init_resource::<resources::Level>()
        .insert_resource(Time::<Fixed>::from_seconds(2.0))
        .insert_state(GameState::Playing)
        .add_systems(
            Startup,
            (setup_camera, spawn_initial_piece, update_gravity_speed),
        )
        .add_systems(
            Update,
            (handle_input, draw_blocks, clear_lines, update_gravity_speed),
        )
        .add_systems(
            FixedUpdate,
            move_piece_down.run_if(in_state(GameState::Playing)),
        )
        .run();
}
