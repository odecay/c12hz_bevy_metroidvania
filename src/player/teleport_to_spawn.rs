use bevy::prelude::*;

use crate::player::Player;


// This is just for testing purposes, press F to teleport to the center of the level, in case you fall of the bottom of the screen

pub fn teleport_to_spawn(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
) {

    for mut transform in query.iter_mut() {


        if keys.pressed(KeyCode::F) {
            transform.translation.x = 1200.0;
            transform.translation.y = 800.0;
        }

    }
}
