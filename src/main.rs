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
        .insert_resource(ChaosFactor(0.5))
        .insert_resource(PreviousChaos(0.5))
        .insert_resource(ZoomLevel(0.5))
        .insert_resource(RenderScale(1.0)) // 1.0 = pleine qualité
        .insert_resource(LoadedChunks::default())

        // 🔌 Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ville Procédurale 3D".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)

        // 🚀 Systèmes de démarrage (Startup)
        .add_systems(Startup, (
            setup_camera,
            display_render_image.after(setup_camera),
            setup_lighting,
            setup_ui,
            spawn_trees,
        ))

        // 🔄 Systèmes de mise à jour (Update)
        .add_systems(Update, (
            egui_chaos_ui,
            manage_chunks,
            pan_camera,
            apply_camera_zoom,
            update_render_texture_on_scale_change,
            display_render_image.after(update_render_texture_on_scale_change),
        ))

        .run();
}
