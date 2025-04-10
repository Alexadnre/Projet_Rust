mod camera;
mod lighting;
mod city;
mod trees;
mod components;
mod ui;
mod chunk;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use camera::{setup_camera, display_render_image, pan_camera, apply_camera_zoom};

use lighting::setup_lighting;
use components::{ChaosFactor, PreviousChaos, ZoomLevel, RenderScale};
use ui::{setup_ui, egui_chaos_ui};
use trees::spawn_trees;
use chunk::{manage_chunks, LoadedChunks};

fn main() {
    App::new()
        // Ressources globales
        .insert_resource(ChaosFactor(0.5))
        .insert_resource(PreviousChaos(0.5))
        .insert_resource(ZoomLevel(0.5))
        .insert_resource(RenderScale(1.0)) // 1.0 = pleine qualité
        .insert_resource(LoadedChunks::default())
        
        // Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ville Procédurale 3D".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)

        // Systèmes de démarrage (Startup)
        .add_systems(Startup, setup_camera)
        // display_render_image doit être exécuté après setup_camera pour que la texture soit prête
        .add_systems(Startup, display_render_image.after(setup_camera))
        .add_systems(Startup, setup_lighting)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, spawn_trees)

        // Systèmes de mise à jour (Update)
        .add_systems(Update, egui_chaos_ui)
        .add_systems(Update, manage_chunks)
        .add_systems(Update, pan_camera)
        .add_systems(Update, apply_camera_zoom)
        
        .run();
}
