use bevy::prelude::*;
use bevy::math::primitives::{Cuboid, Sphere};
use noise::{Perlin, NoiseFn};
use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::components::*;

// Taille d'une tuile dans la grille (en unités Bevy)
pub const TILE_SIZE: f32 = 64.0;
// Taille de la grille (nombre de tuiles par côté)
pub const GRID_SIZE: usize = 31;
// Hauteur maximale des bâtiments (en unités Bevy)
const BUILDING_HEIGHT_LIMIT: f32 = TILE_SIZE * 15.0;
// Probabilité (en pourcentage) de faire apparaître un arbre sur une tuile
const TREE_SPAWN_CHANCE: u8 = 15;

/// Fonction pour supprimer tous les éléments de la ville.
/// Parcourt toutes les entités marquées comme `CityElement` et les supprime.
pub fn clear_city(commands: &mut Commands, query: Query<Entity, With<CityElement>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive(); // Supprime récursivement les entités
    }
}


/// Génération unique de la ville.
/// Utilisée pour créer une ville procédurale au démarrage.
/// - `chaos_value`: Influence le niveau de chaos dans la génération (0.0 = ordonné, 1.0 = chaotique).
pub fn generate_city(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chaos_value: f32,
) {
    // Générateur de nombres aléatoires avec une graine unique
    let seed = rand::thread_rng().gen();
    let mut rng = StdRng::seed_from_u64(seed);
    let perlin = Perlin::new(rng.gen());

    // Grille de la ville, initialisée avec des tuiles vides ('.')
    let mut grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];
    let half_grid = GRID_SIZE as f32 / 2.0;

    // Génération de la grille basée sur le bruit de Perlin
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let noise_value = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
            if noise_value > 0.2 {
                grid[y][x] = '#'; // Marque cette tuile comme un parc
            }
        }
    }

    // Parcours de la grille pour générer les entités
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let pos_x = (x as f32 - half_grid) * TILE_SIZE;
            let pos_z = (y as f32 - half_grid) * TILE_SIZE;

            match grid[y][x] {
                '#' => {
                    // Génère un parc
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(Cuboid::new(TILE_SIZE, 1.0, TILE_SIZE))),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(0.1, 0.8, 0.1), // Couleur verte
                                perceptual_roughness: 0.8,
                                ..default()
                            }),
                            transform: Transform::from_xyz(pos_x, 0.5, pos_z),
                            ..default()
                        },
                        Park,
                        CityElement,
                    ));
                }
                '.' => {
                    if rng.gen_range(0..10) < 7 {
                        // Génère un bâtiment
                        let height = rng.gen_range(
                            TILE_SIZE..=(TILE_SIZE + chaos_value * (BUILDING_HEIGHT_LIMIT - TILE_SIZE))
                        );
                        let width = TILE_SIZE * rng.gen_range(
                            (1.0 - chaos_value * 0.2)..=1.0
                        );
                        let color = match rng.gen_range(0..3) {
                            0 => Color::rgb(0.8, 0.3, 0.3), // Rouge
                            1 => Color::rgb(0.3, 0.3, 0.8), // Bleu
                            _ => Color::rgb(0.8, 0.8, 0.3), // Jaune
                        };

                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(width, height, width))),
                                material: materials.add(StandardMaterial {
                                    base_color: color,
                                    perceptual_roughness: 0.5,
                                    metallic: 0.2,
                                    ..default()
                                }),
                                transform: Transform::from_xyz(pos_x, height / 2.0, pos_z),
                                ..default()
                            },
                            Building,
                            CityElement,
                        ));
                    } else {
                        // Génère une route
                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(TILE_SIZE, 1.0, TILE_SIZE))),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgb(0.2, 0.2, 0.2), // Couleur grise
                                    perceptual_roughness: 0.9,
                                    ..default()
                                }),
                                transform: Transform::from_xyz(pos_x, 0.5, pos_z),
                                ..default()
                            },
                            Road,
                            CityElement,
                        ));
                    }
                }
                _ => {}
            }
        }
    }
}

/// Génération dynamique par chunk.
/// Permet de créer des éléments de la ville par morceaux (chunks) pour une gestion plus efficace.
/// - `chunk_x`: Coordonnée X du chunk.
/// - `chunk_z`: Coordonnée Z du chunk.
/// - `chaos`: Influence le niveau de chaos dans la génération.
/// - `parent`: Entité parent pour regrouper les éléments du chunk.
pub fn generate_chunk_content(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk_x: i32,
    chunk_z: i32,
    chaos: f32,
    parent: Entity,
) {
    // Générateur de nombres aléatoires basé sur les coordonnées du chunk
    let seed = (chunk_x * 73856093) ^ (chunk_z * 19349663);
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let perlin = Perlin::new(rng.gen());

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let world_x = (chunk_x * GRID_SIZE as i32 + x as i32) as f32;
            let world_z = (chunk_z * GRID_SIZE as i32 + y as i32) as f32;

            let pos_x = world_x * TILE_SIZE;
            let pos_z = world_z * TILE_SIZE;

            let noise_value = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
            let tile = if noise_value > 0.2 { '#' } else { '.' };

            match tile {
                '#' => {
                    // Génère un parc
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(Cuboid::new(TILE_SIZE, 1.0, TILE_SIZE))),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(0.1, 0.8, 0.1), // Couleur verte
                                perceptual_roughness: 0.8,
                                ..default()
                            }),
                            transform: Transform::from_xyz(pos_x, 0.5, pos_z),
                            ..default()
                        },
                        Park,
                        CityElement,
                    ))
                    .set_parent(parent); // Associe au parent du chunk
                }
                '.' => {
                    if rng.gen_range(0..10) < 7 {
                        // Génère un bâtiment
                        let height = rng.gen_range(
                            TILE_SIZE..=(TILE_SIZE + chaos * (BUILDING_HEIGHT_LIMIT - TILE_SIZE))
                        );
                        let width = TILE_SIZE * rng.gen_range(
                            (1.0 - chaos * 0.2)..=1.0
                        );
                        let color = match rng.gen_range(0..3) {
                            0 => Color::rgb(0.8, 0.3, 0.3), // Rouge
                            1 => Color::rgb(0.3, 0.3, 0.8), // Bleu
                            _ => Color::rgb(0.8, 0.8, 0.3), // Jaune
                        };

                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(width, height, width))),
                                material: materials.add(StandardMaterial {
                                    base_color: color,
                                    perceptual_roughness: 0.5,
                                    metallic: 0.2,
                                    ..default()
                                }),
                                transform: Transform::from_xyz(pos_x, height / 2.0, pos_z),
                                ..default()
                            },
                            Building,
                            CityElement,
                        ))
                        .set_parent(parent);
                    } else {
                        // Génère une route
                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(TILE_SIZE, 1.0, TILE_SIZE))),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgb(0.2, 0.2, 0.2), // Couleur grise
                                    perceptual_roughness: 0.9,
                                    ..default()
                                }),
                                transform: Transform::from_xyz(pos_x, 0.5, pos_z),
                                ..default()
                            },
                            Road,
                            CityElement,
                        ))
                        .set_parent(parent);
                    }
                }
                _ => {}
            }

            // Génère un arbre aléatoire
            if rng.gen_range(0..100) < TREE_SPAWN_CHANCE {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(Sphere::new(15.0))),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.1, 0.5, 0.1), // Couleur verte foncée
                            ..default()
                        }),
                        transform: Transform::from_xyz(pos_x, 35.0, pos_z),
                        ..default()
                    },
                    CityElement,
                ))
                .set_parent(parent);
            }
        }
    }
}
