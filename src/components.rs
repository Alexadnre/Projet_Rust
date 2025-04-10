// components.rs

use bevy::prelude::*;

// Composants existants
#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct Road;

#[derive(Component)]
pub struct Park;

#[derive(Resource, Clone)]
pub struct ChaosFactor(pub f32);

#[derive(Resource)]
pub struct PreviousChaos(pub f32);

#[derive(Component)]
pub struct CityElement;

#[derive(Resource, Clone)]
pub struct ZoomLevel(pub f32); // Valeur entre 0.5 et 2.0 par exemple

#[derive(Resource, Clone)]
pub struct RenderScale(pub f32); // Valeur entre 0.25 et 1.0 par exemple

use bevy::prelude::*;

#[derive(Resource)]
pub struct RenderTargetInfo {
    pub image_handle: Handle<Image>,
    pub camera_entity: Entity,
}
