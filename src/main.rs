// main.rs

mod camera;
mod lighting;
mod city;
mod trees;
mod components;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use camera::{setup_camera, pan_camera};
use lighting::setup_lighting;
use city::generate_city;
use trees::spawn_trees;
use components::{ChaosFactor, PreviousChaos, CityElement};
use ui::{setup_ui, egui_chaos_ui}; // ✅ egui_chaos_ui à la place de chaos_slider
use components::ZoomLevel;
use camera::apply_camera_zoom;



fn main() {
    App::new()
        .insert_resource(ChaosFactor(0.5))
        .insert_resource(PreviousChaos(0.5))
        .insert_resource(ZoomLevel(0.5)) // ✅ Nouvelle ressource
    
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ville Procédurale 3D".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin) // ✅ Plugin pour interface graphique
        .add_systems(Startup, (
            setup_camera,
            setup_lighting,
            setup_ui,
            spawn_trees,
        ))
        .add_systems(Startup, generate_city_startup)
        .add_systems(Update, pan_camera)
        .add_systems(Update, egui_chaos_ui) // ✅ Slider visuel
        .add_systems(Update, rebuild_city_on_chaos_change)
        .add_systems(Update, apply_camera_zoom)

        .run();
}

/// Système de régénération de la ville si le chaos a changé
fn rebuild_city_on_chaos_change(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    chaos: Res<ChaosFactor>,
    mut prev: ResMut<PreviousChaos>,
    city_query: Query<Entity, With<CityElement>>,
) {
    if (chaos.0 - prev.0).abs() > f32::EPSILON {
        println!("🔁 Régénération de la ville avec chaos {:.2}", chaos.0);

        // Supprime les entités de la ville
        for entity in &city_query {
            commands.entity(entity).despawn_recursive();
        }

        // Reconstruit la ville
        generate_city(commands, meshes, materials, chaos.0);

        // Met à jour le précédent
        prev.0 = chaos.0;
    }
}

/// Génère la ville une première fois au démarrage
fn generate_city_startup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    chaos: Res<ChaosFactor>,
) {
    generate_city(commands, meshes, materials, chaos.0);
}
