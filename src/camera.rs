// camera.rs
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 700.0, 400.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn pan_camera(
    time: Res<Time>, // Ressource pour accéder au temps écoulé entre les frames
    keyboard_input: Res<ButtonInput<KeyCode>>, // Ressource pour détecter les entrées clavier
    mut query: Query<&mut Transform, With<Camera3d>>, // Query pour accéder aux transformations des caméras 3D
) {
    // Calcul de la vitesse de déplacement en fonction du temps écoulé
    let speed = 500.0 * time.delta_seconds();

    // Parcours de toutes les caméras 3D dans la query
    for mut transform in &mut query {
        // Déplacement vers l'avant si la flèche haut est pressée
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.z -= speed; // Réduction de la position Z
        }
        // Déplacement vers l'arrière si la flèche bas est pressée
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.z += speed; // Augmentation de la position Z
        }
        // Déplacement vers la gauche si la flèche gauche est pressée
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed; // Réduction de la position X
        }
        // Déplacement vers la droite si la flèche droite est pressée
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed; // Augmentation de la position X
        }
    }
}
