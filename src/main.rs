// main.rs

// Importation des modules personnalisés
mod camera; // Gestion de la caméra
mod lighting; // Configuration de l'éclairage
mod city; // Génération de la ville procédurale
mod trees; // Génération des arbres
mod components; // Définition des composants ECS

use bevy::prelude::*; // Importation des fonctionnalités principales de Bevy
use camera::{setup_camera, pan_camera}; // Fonctions pour configurer et déplacer la caméra
use lighting::setup_lighting; // Fonction pour configurer l'éclairage
use city::generate_city; // Fonction pour générer la ville
use trees::spawn_trees; // Fonction pour générer les arbres

#[derive(Resource)]
struct ZoomState {
    is_dragging: bool,
    value: f32,
}

fn main() {
    App::new()
        // Ajout des plugins par défaut de Bevy, avec une configuration personnalisée pour la fenêtre
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ville Procédurale 3D".into(), // Titre de la fenêtre
                resolution: (1024.0, 768.0).into(), // Résolution de la fenêtre
                ..default() // Autres paramètres par défaut
            }),
            ..default() // Autres paramètres par défaut pour le plugin de la fenêtre
        }))
        .insert_resource(ZoomState {
            is_dragging: false,
            value: 1.0, // Valeur initiale du zoom
        })
        // Ajout des systèmes à exécuter au démarrage
        .add_systems(Startup, (setup_camera, setup_lighting, generate_city, spawn_trees, setup))
        // Ajout des systèmes à exécuter à chaque mise à jour
        .add_systems(Update, (pan_camera, zoom_system))
        // Lancement de l'application
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 700.0, 400.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Slider pour le zoom
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Zoom: 1.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(50.0),
                top: Val::Px(140.0),
                ..default()
            },
            ..default()
        },
        SliderText,
    ));

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(6.0),
            position_type: PositionType::Absolute,
            right: Val::Px(50.0),
            top: Val::Px(170.0),
            ..default()
        },
        background_color: Color::GRAY.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(12.0),
                    height: Val::Px(24.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(-9.0),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            SliderHandle,
        ));
    });
}

fn zoom_system(
    mut slider_query: Query<&mut Style, With<SliderHandle>>,
    mut text_query: Query<&mut Text, With<SliderText>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut zoom_state: ResMut<ZoomState>,
) {
    let window = windows.single();
    let cursor_pos = window.cursor_position();

    if let Some(cursor_pos) = cursor_pos {
        let bar_width = 150.0;
        let bar_x = window.width() - 200.0;

        for mut style in slider_query.iter_mut() {
            let slider_x = if let Val::Px(x) = style.left { x } else { 0.0 };

            let is_hovering = cursor_pos.x >= (bar_x + slider_x) - 6.0
                && cursor_pos.x <= (bar_x + slider_x) + 6.0
                && cursor_pos.y >= 170.0 - 10.0
                && cursor_pos.y <= 170.0 + 10.0;

            if is_hovering && mouse_input.just_pressed(MouseButton::Left) {
                zoom_state.is_dragging = true;
            }

            if mouse_input.just_released(MouseButton::Left) {
                zoom_state.is_dragging = false;
            }

            if zoom_state.is_dragging {
                let new_x = cursor_pos.x - bar_x;
                let clamped_x = new_x.clamp(0.0, bar_width - 12.0);
                style.left = Val::Px(clamped_x);
                zoom_state.value = 0.5 + (clamped_x / (bar_width - 12.0)) * 2.0; // Zoom entre 0.5 et 2.5
            }
        }
    }

    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Zoom: {:.1}", zoom_state.value);
    }

    for mut transform in camera_query.iter_mut() {
        transform.scale = Vec3::splat(zoom_state.value); // Appliquer le zoom à la caméra
    }
}
