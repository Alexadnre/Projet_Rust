// ui.rs

use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, ButtonInput};
use crate::components::{ChaosFactor};

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });
}

pub fn chaos_slider(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chaos: ResMut<ChaosFactor>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        chaos.0 = (chaos.0 + 0.05).min(1.0);
        println!("Chaos + → {:.2}", chaos.0);
    }
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        chaos.0 = (chaos.0 - 0.05).max(0.0);
        println!("Chaos - → {:.2}", chaos.0);
    }
}
