use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::{ChaosFactor, ZoomLevel}; // ✅ ZoomLevel ajouté ici

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
    mut zoom: ResMut<ZoomLevel>,
) {
    egui::Window::new("🔧 Contrôle Ville").show(contexts.ctx_mut(), |ui| {
        ui.label("Niveau de Chaos");
        let mut chaos_value = chaos.0;
        if ui.add(egui::Slider::new(&mut chaos_value, 0.0..=1.0).text("Chaos")).changed() {
            println!("🌀 Chaos ajusté à {:.2}", chaos_value);
            chaos.0 = chaos_value;
        }

        ui.separator();
        ui.label("Zoom Caméra");
        let mut zoom_value = zoom.0;
        if ui.add(egui::Slider::new(&mut zoom_value, 0.5..=2.0).text("Zoom")).changed() {
            println!("🔍 Zoom ajusté à {:.2}", zoom_value);
            zoom.0 = zoom_value;
        }
    });
}