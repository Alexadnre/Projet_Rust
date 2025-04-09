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

