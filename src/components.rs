// components.rs

use bevy::prelude::*;

/// Composant pour représenter un bâtiment dans la ville.
/// Les bâtiments sont des entités statiques générées dans la grille.
#[derive(Component)]
pub struct Building;

/// Composant pour représenter une route dans la ville.
/// Les routes connectent différentes parties de la ville.
#[derive(Component)]
pub struct Road;

/// Composant pour représenter un parc dans la ville.
/// Les parcs ajoutent des espaces verts à la ville.
#[derive(Component)]
pub struct Park;

/// Ressource pour stocker le niveau de chaos.
/// Le chaos influence la génération procédurale de la ville.
/// Valeur comprise entre 0.0 (aucun chaos) et 1.0 (chaos maximal).
#[derive(Resource, Clone)]
pub struct ChaosFactor(pub f32);

/// Ressource pour stocker la valeur précédente du chaos.
/// Utile pour détecter les changements dans le niveau de chaos.
#[derive(Resource)]
pub struct PreviousChaos(pub f32);

/// Composant générique pour identifier les éléments de la ville.
/// Utilisé pour regrouper les entités liées à la ville.
#[derive(Component)]
pub struct CityElement;

/// Ressource pour gérer le niveau de zoom de la caméra.
/// Valeur typique entre 0.5 (zoom arrière) et 2.0 (zoom avant).
#[derive(Resource, Clone)]
pub struct ZoomLevel(pub f32);

/// Ressource pour gérer l'échelle de rendu.
/// Contrôle la qualité de rendu, avec des valeurs typiques entre 0.25 (qualité basse) et 1.0 (qualité maximale).
#[derive(Resource, Clone)]
pub struct RenderScale(pub f32);

/// Ressource pour stocker les informations sur la cible de rendu.
/// Contient le handle de l'image de rendu et l'entité de la caméra associée.
#[derive(Resource)]
pub struct RenderTargetInfo {
    pub image_handle: Handle<Image>, // Handle vers l'image utilisée pour le rendu
    pub camera_entity: Entity, // Entité de la caméra associée à cette cible
}
