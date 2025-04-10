use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::{ChaosFactor, ZoomLevel, RenderScale};

/// Initialise la caméra 2D pour l’interface utilisateur.
/// Cette caméra est utilisée pour afficher les éléments de l'interface graphique.
/// - `commands`: Permet de créer et gérer des entités dans le monde.
pub fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1, // Définit l'ordre de rendu pour superposer l'interface sur la scène 3D
            ..default()
        },
        ..default()
    });
}

/// Interface utilisateur pour ajuster les paramètres.
/// Permet de modifier dynamiquement le niveau de chaos, le zoom de la caméra et la qualité de rendu.
/// - `contexts`: Contexte Egui pour dessiner l'interface.
/// - `chaos`: Ressource mutable pour ajuster le niveau de chaos.
/// - `zoom`: Ressource mutable pour ajuster le niveau de zoom.
/// - `render_scale`: Ressource mutable pour ajuster la qualité de rendu.
pub fn egui_chaos_ui(
    mut contexts: EguiContexts,
    mut chaos: ResMut<ChaosFactor>,
    mut zoom: ResMut<ZoomLevel>,
    mut render_scale: ResMut<RenderScale>,
) {
    egui::Window::new("🔧 Contrôle Ville").show(contexts.ctx_mut(), |ui| {
        // Slider pour ajuster le niveau de chaos
        ui.label("Niveau de Chaos");
        let mut chaos_value = chaos.0;
        let response = ui.add(
            egui::Slider::new(&mut chaos_value, 0.0..=1.0)
                .text("Chaos")
                .clamp_to_range(true),
        );
        if response.drag_released() { // Applique le changement uniquement après avoir relâché la souris -> pour les performances
            if (chaos_value - chaos.0).abs() > f32::EPSILON {
                println!("🌀 Chaos ajusté à {:.2}", chaos_value);
                chaos.0 = chaos_value;
            }
        }

        ui.separator(); // Séparateur visuel
        // Slider pour ajuster le zoom de la caméra
        ui.label("Zoom Caméra");
        let mut zoom_value = zoom.0;
        if ui
            .add(egui::Slider::new(&mut zoom_value, 0.1..=2.0).text("Zoom"))
            .changed()
        {
            println!("🔍 Zoom ajusté à {:.2}", zoom_value);
            zoom.0 = zoom_value;
        }

        ui.separator();
        // Slider pour ajuster la qualité de rendu
        ui.label("Qualité de rendu");
        let mut scale_value = render_scale.0;
        if ui
            .add(egui::Slider::new(&mut scale_value, 0.05..=1.0).text("Résolution"))
            .changed()
        {
            println!("🖼️ Résolution de rendu ajustée à {:.2}", scale_value);
            render_scale.0 = scale_value;
        }
    });
}
