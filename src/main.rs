// main.rs

mod camera;
mod lighting;
mod city;
mod trees;
mod components;
mod ui;
mod chunk; // ✅ Module chunk

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use camera::{setup_camera, pan_camera, apply_camera_zoom};
use lighting::setup_lighting;
use components::{ChaosFactor, PreviousChaos, ZoomLevel};
use ui::{setup_ui, egui_chaos_ui};
use trees::spawn_trees;
use chunk::{manage_chunks, LoadedChunks}; // ✅ Import chunk système

fn main() {
    App::new()
        // 🧠 Ressources globales
        .insert_resource(ChaosFactor(0.5))
        .insert_resource(PreviousChaos(0.5))
        .insert_resource(ZoomLevel(0.5))
        .insert_resource(LoadedChunks::default()) // ✅ Chunks chargés

        // 🔌 Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ville Procédurale 3D".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin) // ✅ Interface graphique EGUI

        // 🚀 Systèmes de démarrage
        .add_systems(Startup, (
            setup_camera,
            setup_lighting,
            setup_ui,
            spawn_trees, // (optionnel) : arbres initiaux à la racine
        ))

        // 🔁 Systèmes de mise à jour
        .add_systems(Update, pan_camera)
        .add_systems(Update, egui_chaos_ui)
        .add_systems(Update, apply_camera_zoom)
        .add_systems(Update, manage_chunks) // ✅ Système chunk infini & suppression

        .run();
}
