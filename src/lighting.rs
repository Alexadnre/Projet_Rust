// lighting.rs

use bevy::prelude::*;

/// Configure l'éclairage de la scène.
/// Ajoute une lumière directionnelle pour illuminer la ville.
/// - `commands`: Permet de créer et gérer des entités dans le monde.
pub fn setup_lighting(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        // Lumière directionnelle avec une intensité modérée
        directional_light: DirectionalLight {
            illuminance: 10_000.0, // Intensité lumineuse
            shadows_enabled: true, // Active les ombres
            ..default()
        },
        // Position et orientation de la lumière
        transform: Transform::from_xyz(0.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ajout d'une lumière ambiante pour éclairer uniformément la scène
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.9, 0.9, 0.9), // Couleur blanche légèrement atténuée
        brightness: 0.7, // Intensité de la lumière ambiante
    });
}