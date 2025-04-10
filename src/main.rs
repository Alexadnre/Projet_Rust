mod camera;
mod lighting;
mod city;
mod trees;
mod components;
mod ui;
mod chunk;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use camera::{setup_camera, display_render_image, pan_camera, apply_camera_zoom, update_render_texture_on_scale_change};
use lighting::setup_lighting;
use components::{ChaosFactor, PreviousChaos, ZoomLevel, RenderScale};
use ui::{setup_ui, egui_chaos_ui};
use trees::spawn_trees;
use chunk::{manage_chunks, LoadedChunks};

fn main() {
    App::new()
        // 💾 Ressources globales
        .insert_resource(ChaosFactor(0.5)) // Ressource représentant le niveau de chaos initial dans le système
        .insert_resource(PreviousChaos(0.5)) // Ressource pour sauvegarder le niveau de chaos précédent
        .insert_resource(ZoomLevel(0.5)) // Ressource définissant le niveau de zoom initial de la caméra
        .insert_resource(RenderScale(1.0)) // Ressource pour la qualité de rendu initiale (pleine qualité)
        .insert_resource(LoadedChunks::default()) // Ressource pour gérer les chunks chargés dans le monde

        // 🔌 Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ville Procédurale 3D".into(), // Titre de la fenêtre de l'application
                resolution: (1024.0, 768.0).into(), // Résolution initiale de la fenêtre
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin) // Plugin pour intégrer l'interface utilisateur Egui

        // 🚀 Systèmes de démarrage (Startup)
        .add_systems(Startup, (
            setup_camera, // Configuration initiale de la caméra 3D
            display_render_image.after(setup_camera), // Affichage de la texture de rendu après la configuration de la caméra
            setup_lighting, // Configuration des paramètres d'éclairage de la scène
            setup_ui, // Initialisation des éléments de l'interface utilisateur
            spawn_trees, // Génération des arbres dans la scène
        ))

        // 🔄 Systèmes de mise à jour (Update)
        .add_systems(Update, (
            egui_chaos_ui, // Interface utilisateur permettant d'ajuster les paramètres de chaos
            manage_chunks, // Gestion dynamique des chunks en fonction de la position de la caméra
            pan_camera, // Déplacement de la caméra dans la scène
            apply_camera_zoom, // Application des changements de zoom de la caméra
            update_render_texture_on_scale_change, // Mise à jour de la texture de rendu en cas de changement de qualité
            display_render_image.after(update_render_texture_on_scale_change), // Affichage de la texture mise à jour après modification
        ))

        .run(); // Démarrage de l'application principale
}
