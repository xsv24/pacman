use crate::player::Pacman;
use bevy::prelude::*;

pub fn pacman_controller(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Pacman, &mut Transform)>,
) {
    let mut horizontal_direction = 0.0;
    let mut vertical_direction = 0.0;

    for (pacman, mut transform) in query.iter_mut() {
        if input.pressed(KeyCode::Left) {
            horizontal_direction -= 1.0;
        } else if input.pressed(KeyCode::Right) {
            horizontal_direction += 1.0;
        } else if input.pressed(KeyCode::Up) {
            vertical_direction += 1.0;
        } else if input.pressed(KeyCode::Down) {
            vertical_direction -= 1.0;
        }

        let translation = &mut transform.translation;

        *translation.x_mut() += time.delta_seconds * horizontal_direction * pacman.speed;
        *translation.y_mut() += time.delta_seconds * vertical_direction * pacman.speed;
    }
}
