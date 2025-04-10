use bevy::prelude::*;
use std::collections::HashSet;

use crate::components::{CityElement, ChaosFactor};
use crate::city::generate_chunk_content;

/// Taille d'un chunk en nombre de tuiles.
pub const CHUNK_SIZE: i32 = 31;
/// Taille d'une tuile en unités de monde.
pub const TILE_SIZE: f32 = 64.0;
/// Taille d'un chunk en unités de monde.
pub const CHUNK_WORLD_SIZE: f32 = CHUNK_SIZE as f32 * TILE_SIZE;
/// Distance de vue en chunks autour de la caméra.
const VIEW_DISTANCE: i32 = 2;
/// Distance au-delà de laquelle les chunks sont déchargés.
const UNLOAD_DISTANCE: i32 = 4;

/// Ressource pour gérer les chunks chargés.
/// Contient un ensemble des coordonnées des chunks actifs et leurs entités associées.
#[derive(Resource)]
pub struct LoadedChunks {
    pub set: HashSet<(i32, i32)>, // Ensemble des coordonnées des chunks chargés
    pub entities: Vec<(Entity, (i32, i32))>, // Liste des entités racines des chunks et leurs coordonnées
    pub last_chaos: f32, // Dernier niveau de chaos utilisé pour la génération
}

impl Default for LoadedChunks {
    fn default() -> Self {
        Self {
            set: HashSet::new(),
            entities: vec![],
            last_chaos: 0.5,
        }
    }
}

/// Gère dynamiquement les chunks visibles autour de la caméra.
/// Charge les chunks proches et décharge ceux qui sont trop éloignés.
/// - `commands`: Permet de créer et gérer des entités dans le monde.
/// - `camera_query`: Permet de récupérer la position de la caméra.
/// - `meshes`: Gestion des maillages 3D.
/// - `materials`: Gestion des matériaux appliqués aux maillages.
/// - `chaos`: Niveau de chaos influençant la génération.
/// - `loaded`: Ressource mutable pour suivre les chunks chargés.
pub fn manage_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera3d>>, // Position de la caméra
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chaos: Res<ChaosFactor>, // Niveau de chaos influençant la génération
    mut loaded: ResMut<LoadedChunks>, // Gestion des chunks chargés
) {
    let cam_pos = camera_query.single().translation; // Position actuelle de la caméra
    let cam_chunk_x = (cam_pos.x / CHUNK_WORLD_SIZE).floor() as i32; // Chunk X de la caméra
    let cam_chunk_z = (cam_pos.z / CHUNK_WORLD_SIZE).floor() as i32; // Chunk Z de la caméra

    // Si le niveau de chaos a changé, réinitialise tous les chunks
    if (chaos.0 - loaded.last_chaos).abs() > f32::EPSILON {
        println!("🌀 Chaos modifié → suppression et rechargement des chunks");
        for (entity, _) in &loaded.entities {
            commands.entity(*entity).despawn_recursive(); // Supprime récursivement les entités du chunk
        }
        loaded.set.clear(); // Vide la liste des chunks chargés
        loaded.entities.clear(); // Vide les entités associées
        loaded.last_chaos = chaos.0; // Met à jour le niveau de chaos
    }

    // Charge les chunks visibles autour de la caméra
    for dz in -VIEW_DISTANCE..=VIEW_DISTANCE {
        for dx in -VIEW_DISTANCE..=VIEW_DISTANCE {
            let cx = cam_chunk_x + dx;
            let cz = cam_chunk_z + dz;
            let coord = (cx, cz);

            if !loaded.set.contains(&coord) {
                let chunk_root = commands.spawn(SpatialBundle::default()).id(); // Crée un parent pour le chunk
                generate_chunk_content(&mut commands, &mut meshes, &mut materials, cx, cz, chaos.0, chunk_root);
                loaded.set.insert(coord); // Ajoute le chunk à la liste des chargés
                loaded.entities.push((chunk_root, coord)); // Associe l'entité au chunk
            }
        }
    }

    // Décharge les chunks trop éloignés
    let mut retained_entities = vec![];
    let mut retained_coords = HashSet::new();

    for (entity, (x, z)) in &loaded.entities {
        let dx = cam_chunk_x - *x;
        let dz = cam_chunk_z - *z;
        let dist = dx.abs().max(dz.abs());

        if dist > UNLOAD_DISTANCE {
            commands.entity(*entity).despawn_recursive(); // Supprime le chunk
        } else {
            retained_entities.push((*entity, (*x, *z))); // Conserve le chunk
            retained_coords.insert((*x, *z)); // Conserve les coordonnées
        }
    }

    loaded.entities = retained_entities; // Met à jour les entités conservées
    loaded.set = retained_coords; // Met à jour les coordonnées conservées
}
