// components.rs

use bevy::prelude::*;

// Composant pour identifier une entité comme un bâtiment
#[derive(Component)]
pub struct Building;

// Composant pour identifier une entité comme une route
#[derive(Component)]
pub struct Road;

// Composant pour identifier une entité comme un parc
#[derive(Component)]
pub struct Park;