use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui}; // ✅ Import du module EGUI
use crate::components::ChaosFactor;

/// Initialise la caméra 2D pour l’interface utilisateur
pub fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });
}

/// Affiche un slider visuel pour ajuster le niveau de chaos
pub fn egui_chaos_ui(
    mut contexts: EguiContexts,
    mut chaos: ResMut<ChaosFactor>,
) {
    egui::Window::new("🔧 Chaos Control").show(contexts.ctx_mut(), |ui| {
        ui.label("Niveau de Chaos");
        let mut value = chaos.0;
        if ui
            .add(egui::Slider::new(&mut value, 0.0..=1.0).text("Chaos"))
            .changed()
        {
            println!("🌀 Chaos ajusté à {:.2}", value);
            chaos.0 = value;
        }
    });
}
