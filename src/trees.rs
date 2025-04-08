// trees.rs

use bevy::prelude::*;
use rand::Rng;

// Taille d'une tuile dans la grille (en unités Bevy)
const TILE_SIZE: f32 = 64.0;
// Taille de la grille (nombre de tuiles par côté)
const GRID_SIZE: usize = 31;
// Probabilité (en pourcentage) de faire apparaître un arbre sur une tuile
const TREE_SPAWN_CHANCE: u8 = 15;

pub fn spawn_trees(
    mut commands: Commands, // Permet de créer et gérer des entités dans le monde
    mut meshes: ResMut<Assets<Mesh>>, // Gestion des maillages 3D
    mut materials: ResMut<Assets<StandardMaterial>>, // Gestion des matériaux appliqués aux maillages
) {
    let mut rng = rand::thread_rng(); // Générateur de nombres aléatoires

    // Parcours de chaque tuile de la grille
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            // Génère un nombre aléatoire pour décider si un arbre doit être placé
            if rng.gen_range(0..100) < TREE_SPAWN_CHANCE {
                // Calcul des coordonnées en fonction de la position dans la grille
                let pos_x = (x as f32 - GRID_SIZE as f32 / 2.0) * TILE_SIZE;
                let pos_z = (y as f32 - GRID_SIZE as f32 / 2.0) * TILE_SIZE;

                // Création d'une entité représentant un arbre
                commands.spawn(PbrBundle {
                    // Ajout d'un maillage en forme de sphère pour représenter la cime de l'arbre
                    mesh: meshes.add(Mesh::from(shape::UVSphere { 
                        radius: 15.0, // Rayon de la sphère (taille de la cime)
                        ..default() // Autres paramètres par défaut
                    })),
                    // Ajout d'un matériau vert pour représenter les feuilles
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.1, 0.5, 0.1), // Couleur verte
                        ..default() // Autres propriétés par défaut
                    }),
                    // Positionnement de l'arbre dans le monde
                    transform: Transform::from_xyz(pos_x, 35.0, pos_z), // Hauteur de 35.0 pour la cime
                    ..default() // Autres paramètres par défaut
                });
            }
        }
    }
}
