// city.rs
use bevy::prelude::*;
use bevy::render::mesh::shape::Box;
use crate::components::ChaosFactor;


use noise::{Perlin, NoiseFn};
use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::components::*;

const TILE_SIZE: f32 = 64.0;
const GRID_SIZE: usize = 31;
const BUILDING_HEIGHT_LIMIT: f32 = TILE_SIZE * 15.0;
// Fonction pour supprimer la ville
pub fn clear_city(mut commands: Commands, query: Query<Entity, With<CityElement>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
// Fonction pour générer la ville procéduralement
pub fn generate_city(
    mut commands: Commands, // Permet de créer et gérer des entités dans le monde
    mut meshes: ResMut<Assets<Mesh>>, // Gestion des maillages 3D
    mut materials: ResMut<Assets<StandardMaterial>>, // Gestion des matériaux appliqués aux maillages
    chaos_value: f32, // Facteur de chaos pour influencer la génération avec le slider
) {
    // Génération d'une graine aléatoire pour le générateur de nombres
    let seed = rand::thread_rng().gen();
    let mut rng = StdRng::seed_from_u64(seed); // Initialisation du générateur avec la graine
    let perlin = Perlin::new(rng.gen()); // Création d'une instance de bruit Perlin

    // Initialisation de la grille avec des tuiles par défaut ('.')
    let mut grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];

    // Calcul de la moitié de la taille de la grille pour centrer les coordonnées
    let half_grid = GRID_SIZE as f32 / 2.0;

    // Génération des parcs avec le bruit Perlin
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            // Calcul de la valeur de bruit pour la position actuelle
            let noise_value = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
            if noise_value > 0.2 {
                grid[y][x] = '#'; // Marque la tuile comme un parc ('#')
            }
        }
    }

    // Parcours de chaque tuile de la grille pour générer les entités correspondantes
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            // Calcul des coordonnées en fonction de la position dans la grille
            let pos_x = (x as f32 - half_grid) * TILE_SIZE; // Position X dans le monde
            let pos_z = (y as f32 - half_grid) * TILE_SIZE; // Position Z dans le monde

            // Vérifie le type de terrain sur la tuile actuelle
            match grid[y][x] {
                '#' => {
                    // Si la tuile est un parc ('#'), crée une entité avec un matériau vert
                    commands.spawn((
                        PbrBundle {
                            // Ajout d'un maillage en forme de boîte pour représenter le parc
                            mesh: meshes.add(Mesh::from(Box::new(TILE_SIZE, 1.0, TILE_SIZE))),
                            // Ajout d'un matériau vert vif pour représenter l'herbe
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(0.1, 0.8, 0.1), // Couleur verte
                                perceptual_roughness: 0.8, // Apparence rugueuse pour l'herbe
                                ..default() // Autres propriétés par défaut
                            }),
                            // Positionnement du parc dans le monde
                            transform: Transform::from_xyz(pos_x, 0.5, pos_z), // Hauteur de 0.5 pour l'élévation
                            ..default() // Autres paramètres par défaut
                        },
                        Park, // Ajout du composant `Park` pour identifier cette entité comme un parc
                        CityElement,// Ajout du composant `CityElement` pour identifier cette entité comme un élément de la ville
                    ));
                }
                '.' => {
                    // Si la tuile est un bâtiment ou une route
                    if rng.gen_range(0..10) < 7 {
                        // Génération d'un bâtiment
                        let chaos_factor = chaos_value;
                        let height = rng.gen_range(
                            TILE_SIZE..=(TILE_SIZE + chaos_factor * (BUILDING_HEIGHT_LIMIT - TILE_SIZE))
                        );// Hauteur variable en fonction du facteur de chaos
                        let width = TILE_SIZE * rng.gen_range(
                            (1.0 - chaos_factor * 0.2)..=1.0
                        );// Largeur variable en fonction du facteur de chaos
                        let color = match rng.gen_range(0..3) {
                            0 => Color::rgb(0.8, 0.3, 0.3), // Rouge brique
                            1 => Color::rgb(0.3, 0.3, 0.8), // Bleu
                            _ => Color::rgb(0.8, 0.8, 0.3), // Jaune pâle
                        };

                        commands.spawn((
                            PbrBundle {
                                // Ajout d'un maillage en forme de boîte pour représenter le bâtiment
                                mesh: meshes.add(Mesh::from(Box::new(width as f32, height as f32, width as f32))),
                                // Ajout d'un matériau coloré pour le bâtiment
                                material: materials.add(StandardMaterial {
                                    base_color: color,
                                    perceptual_roughness: 0.5, // Apparence légèrement rugueuse
                                    metallic: 0.2, // Apparence métallique
                                    ..default()
                                }),
                                // Positionnement du bâtiment dans le monde
                                transform: Transform::from_xyz(pos_x, height / 2.0, pos_z), // Centré sur la hauteur
                                ..default()
                            },
                            Building, // Ajout du composant `Building` pour identifier cette entité comme un bâtiment
                            CityElement, // Ajout du composant `CityElement` pour identifier cette entité comme un élément de la ville
                        ));
                    } else {
                        // Génération d'une route
                        commands.spawn((
                            PbrBundle {
                                // Ajout d'un maillage en forme de boîte pour représenter la route
                                mesh: meshes.add(Mesh::from(Box::new(TILE_SIZE, 1.0, TILE_SIZE))),
                                // Ajout d'un matériau gris foncé pour représenter l'asphalte
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgb(0.2, 0.2, 0.2), // Gris foncé
                                    perceptual_roughness: 0.9, // Apparence très rugueuse
                                    ..default()
                                }),
                                // Positionnement de la route dans le monde
                                transform: Transform::from_xyz(pos_x, 0.5, pos_z), // Hauteur de 0.5 pour l'élévation
                                ..default()
                            },
                            Road, // Ajout du composant `Road` pour identifier cette entité comme une route
                            CityElement, // Ajout du composant `CityElement` pour identifier cette entité comme un élément de la ville
                        ));
                    }
                }
                _ => {
                    // Si la tuile ne correspond à aucun type connu, ne rien faire
                }
            }
        }
    }
}
