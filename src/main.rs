mod common;
mod player;

use crate::player::Pacman;
use bevy::prelude::*;

fn init_game_state(mut commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    let command_chain = commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());

    Pacman::attach(command_chain, materials);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(init_game_state.system())
        .add_system(common::pacman_controller.system())
        .run();
}
