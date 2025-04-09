use bevy::prelude::*;
use std::collections::HashSet;

use crate::components::{CityElement, ChaosFactor};
use crate::city::generate_chunk_content;

pub const CHUNK_SIZE: i32 = 31;
pub const TILE_SIZE: f32 = 64.0;
pub const CHUNK_WORLD_SIZE: f32 = CHUNK_SIZE as f32 * TILE_SIZE;
const VIEW_DISTANCE: i32 = 2;
const UNLOAD_DISTANCE: i32 = 4;

#[derive(Resource)]
pub struct LoadedChunks {
    pub set: HashSet<(i32, i32)>,
    pub entities: Vec<(Entity, (i32, i32))>, // root entity ↔ coord chunk
    pub last_chaos: f32,
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

pub fn manage_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chaos: Res<ChaosFactor>,
    mut loaded: ResMut<LoadedChunks>,
) {
    let cam_pos = camera_query.single().translation;
    let cam_chunk_x = (cam_pos.x / CHUNK_WORLD_SIZE).floor() as i32;
    let cam_chunk_z = (cam_pos.z / CHUNK_WORLD_SIZE).floor() as i32;

    // 🔁 Si le chaos a changé, reset complet
    if (chaos.0 - loaded.last_chaos).abs() > f32::EPSILON {
        println!("🌀 Chaos modifié → suppression et rechargement des chunks");

        for (entity, _) in &loaded.entities {
            commands.entity(*entity).despawn_recursive(); // détruit tout le contenu du chunk
        }

        loaded.set.clear();
        loaded.entities.clear();
        loaded.last_chaos = chaos.0;
    }

    // 🔁 Chargement des chunks visibles
    for dz in -VIEW_DISTANCE..=VIEW_DISTANCE {
        for dx in -VIEW_DISTANCE..=VIEW_DISTANCE {
            let cx = cam_chunk_x + dx;
            let cz = cam_chunk_z + dz;
            let coord = (cx, cz);

            if !loaded.set.contains(&coord) {
                let chunk_root = commands.spawn(SpatialBundle::default()).id(); // root parent
                generate_chunk_content(&mut commands, &mut meshes, &mut materials, cx, cz, chaos.0, chunk_root);
                loaded.set.insert(coord);
                loaded.entities.push((chunk_root, coord));
            }
        }
    }

    // ❌ Suppression des chunks hors distance
    let mut retained_entities = vec![];
    let mut retained_coords = HashSet::new();

    for (entity, (x, z)) in &loaded.entities {
        let dx = cam_chunk_x - *x;
        let dz = cam_chunk_z - *z;
        let dist = dx.abs().max(dz.abs());

        if dist > UNLOAD_DISTANCE {
            commands.entity(*entity).despawn_recursive();
        } else {
            retained_entities.push((*entity, (*x, *z)));
            retained_coords.insert((*x, *z));
        }
    }

    loaded.entities = retained_entities;
    loaded.set = retained_coords;
}
