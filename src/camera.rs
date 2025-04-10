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
pub struct RenderImage(pub Handle<Image>);

/// Crée une caméra 3D qui rend dans une texture
pub fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    render_scale: Res<RenderScale>,
    windows: Query<&Window>,
) {
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
        // Utilise le sampler "nearest" pour garder un rendu avec effet pixelisé si besoin
        sampler: bevy::render::texture::ImageSampler::nearest(),
        ..default()
    };

    let image_handle = images.add(image);

    // Modification de la cible de rendu : la caméra rend dans la texture "image_handle"
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 700.0, 400.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RenderImage(image_handle),
    ));
}

/// Affiche dynamiquement la texture de rendu (RenderImage) via un sprite
pub fn display_render_image(
    mut commands: Commands,
    render_image_query: Query<&RenderImage>,
) {
    info!("display_render_image: Tentative de création du sprite");
    if let Some(render_image) = render_image_query.iter().next() {
        commands.spawn(SpriteBundle {
            texture: render_image.0.clone(),
            transform: Transform {
                // Positionne le sprite au centre de la scène
                translation: Vec3::new(0.0, 0.0, 0.0),
                // Ajuste l'échelle pour agrandir l'affichage de la texture
                scale: Vec3::splat(5.0),
                ..default()
            },
            ..default()
        });
        info!("display_render_image: Sprite créé avec la texture {:?}", render_image.0);

    }
}

/// Permet de déplacer la caméra avec les touches fléchées
pub fn pan_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let speed = 500.0 * time.delta_seconds();

    for mut transform in &mut query {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.z -= speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.z += speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed;
        }
    }
}

/// Applique dynamiquement le zoom selon la valeur du slider UI
pub fn apply_camera_zoom(
    zoom: Res<ZoomLevel>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    if zoom.is_changed() {
        for mut transform in &mut query {
            transform.translation.y = 700.0 / zoom.0;
            transform.translation.z = 400.0 / zoom.0;
        }
    }
}
