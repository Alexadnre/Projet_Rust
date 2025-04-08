// lighting.rs

use bevy::prelude::*;

// Fonction pour configurer l'éclairage de la scène
pub fn setup_lighting(mut commands: Commands) {
    // Ajout d'une lumière directionnelle pour simuler la lumière du soleil
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10_000.0, // Intensité de la lumière (en lumens)
            shadows_enabled: true, // Active les ombres projetées par cette lumière
            ..default() // Autres propriétés par défaut
        },
        transform: Transform::from_xyz(0.0, 700.0, 50.0) // Position de la lumière dans la scène
            .looking_at(Vec3::ZERO, Vec3::Y), // Orientation de la lumière vers le centre de la scène
        ..default() // Autres paramètres par défaut
    });

    // Ajout d'une lumière ambiante pour éclairer uniformément la scène
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.9, 0.9, 0.9), // Couleur blanche légèrement atténuée
        brightness: 0.7, // Intensité de la lumière ambiante
    });
}