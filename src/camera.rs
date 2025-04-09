use bevy::prelude::*;
use crate::components::ZoomLevel;

/// Initialise la caméra 3D positionnée au-dessus de la ville
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 700.0, 400.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Permet de déplacer la caméra avec les touches fléchées
pub fn pan_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let speed = 500.0 * time.delta_seconds();

    for mut transform in &mut query {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.z -= speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.z += speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed;
        }
    }
}

/// Applique dynamiquement le zoom selon la valeur du slider EGUI
pub fn apply_camera_zoom(
    zoom: Res<ZoomLevel>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    if zoom.is_changed() {
        for mut transform in &mut query {
            // Le zoom modifie la hauteur (Y) et la profondeur (Z) de la caméra
            transform.translation.y = 700.0 / zoom.0;
            transform.translation.z = 400.0 / zoom.0;
        }
    }
}
