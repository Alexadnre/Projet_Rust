// camera.rs

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::texture::Image;
use bevy::math::Vec3;
use bevy::window::Window;

use crate::components::{RenderScale, ZoomLevel};

#[derive(Component)]
pub struct RenderSprite;

#[derive(Resource)]
pub struct RenderTargetInfo {
    pub image_handle: Handle<Image>,
    pub camera_entity: Entity,
}

/// Crée une caméra 3D qui rend dans une texture.
/// Cette caméra est utilisée pour capturer la scène et la rendre dans une texture spécifique.
pub fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    render_scale: Res<RenderScale>, // Échelle de rendu pour ajuster la résolution
    windows: Query<&Window>, // Fenêtre principale pour récupérer ses dimensions
) {
    let window = windows.single();
    let (width, height) = (window.resolution.width(), window.resolution.height());

    let target_width = (width * render_scale.0) as u32;
    let target_height = (height * render_scale.0) as u32;

    let image = Image {
        // Crée une texture vide avec les dimensions calculées
        data: vec![0; (target_width * target_height * 4) as usize],
        texture_descriptor: TextureDescriptor {
            size: Extent3d {
                width: target_width,
                height: target_height,
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            label: None,
            view_formats: &[],
        },
        sampler: bevy::render::texture::ImageSampler::nearest(), // Utilise un échantillonnage proche
        ..default()
    };

    let image_handle = images.add(image);

    let camera_entity = commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 700.0, 400.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
        ))
        .id();

    commands.insert_resource(RenderTargetInfo {
        image_handle: image_handle.clone(), // Stocke le handle de l'image
        camera_entity, // Stocke l'entité de la caméra
    });
}

/// Met à jour dynamiquement la texture de rendu lorsque l'échelle de rendu change.
/// Cela permet d'ajuster la résolution de la texture en fonction des besoins.
pub fn update_render_texture_on_scale_change(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut render_target_info: ResMut<RenderTargetInfo>,
    render_scale: Res<RenderScale>, // Nouvelle échelle de rendu
    windows: Query<&Window>, // Fenêtre principale pour récupérer ses dimensions
    mut camera_query: Query<&mut Camera>, // Caméra à mettre à jour
) {
    if !render_scale.is_changed() {
        return; // Ne fait rien si l'échelle n'a pas changé
    }

    let window = windows.single();
    let (width, height) = (window.resolution.width(), window.resolution.height());
    let target_width = (width * render_scale.0) as u32;
    let target_height = (height * render_scale.0) as u32;

    let image = Image {
        data: vec![0; (target_width * target_height * 4) as usize],
        texture_descriptor: TextureDescriptor {
            size: Extent3d {
                width: target_width,
                height: target_height,
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            label: None,
            view_formats: &[],
        },
        sampler: bevy::render::texture::ImageSampler::nearest(),
        ..default()
    };

    let new_image_handle = images.add(image);

    if let Ok(mut camera) = camera_query.get_mut(render_target_info.camera_entity) {
        camera.target = RenderTarget::Image(new_image_handle.clone()); // Met à jour la cible de rendu
    }

    render_target_info.image_handle = new_image_handle; // Met à jour le handle de l'image
}

/// Affiche dynamiquement la texture de rendu via un sprite.
/// Ce sprite est utilisé pour visualiser la texture de rendu dans la fenêtre.
pub fn display_render_image(
    mut commands: Commands,
    render_target: Res<RenderTargetInfo>, // Informations sur la cible de rendu
    render_scale: Res<RenderScale>, // Échelle de rendu actuelle
    windows: Query<&Window>, // Fenêtre principale pour récupérer ses dimensions
    old_sprite_query: Query<Entity, With<RenderSprite>>, // Ancien sprite à supprimer
) {
    if render_scale.is_changed() {
        for entity in old_sprite_query.iter() {
            commands.entity(entity).despawn(); // Supprime l'ancien sprite
        }

        let window = windows.single();
        let window_size = Vec2::new(window.resolution.width(), window.resolution.height());

        commands.spawn((
            SpriteBundle {
                texture: render_target.image_handle.clone(), // Utilise la texture de rendu
                sprite: Sprite {
                    custom_size: Some(window_size), // Ajuste la taille du sprite à la fenêtre
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO), // Positionne le sprite au centre
                ..default()
            },
            RenderSprite, // Marque l'entité comme un sprite de rendu
        ));
    }
}

/// Permet de déplacer la caméra avec les touches fléchées.
/// Ce système ajuste la position de la caméra en fonction des entrées utilisateur.
pub fn pan_camera(
    time: Res<Time>, // Temps écoulé pour un déplacement fluide
    keyboard_input: Res<ButtonInput<KeyCode>>, // Entrées clavier
    mut query: Query<&mut Transform, With<Camera3d>>, // Transformations de la caméra
) {
    let speed = 500.0 * time.delta_seconds(); // Vitesse de déplacement

    for mut transform in &mut query {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.z -= speed; // Déplace la caméra vers l'avant
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.z += speed; // Déplace la caméra vers l'arrière
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed; // Déplace la caméra vers la gauche
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed; // Déplace la caméra vers la droite
        }
    }
}

/// Applique un zoom dynamique à la caméra 3D.
/// Ajuste la position de la caméra en fonction du niveau de zoom.
pub fn apply_camera_zoom(
    zoom: Res<ZoomLevel>, // Niveau de zoom actuel
    mut query: Query<&mut Transform, With<Camera3d>>, // Transformations de la caméra
) {
    if zoom.is_changed() {
        for mut transform in &mut query {
            transform.translation.y = 700.0 / zoom.0; // Ajuste la hauteur en fonction du zoom
            transform.translation.z = 400.0 / zoom.0; // Ajuste la profondeur en fonction du zoom
        }
    }
}
