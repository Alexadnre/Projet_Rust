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
        sampler: bevy::render::texture::ImageSampler::nearest(),
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
        image_handle: image_handle.clone(),
        camera_entity,
    });
}

/// Met à jour dynamiquement la texture de rendu quand RenderScale change
pub fn update_render_texture_on_scale_change(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut render_target_info: ResMut<RenderTargetInfo>,
    render_scale: Res<RenderScale>,
    windows: Query<&Window>,
    mut camera_query: Query<&mut Camera>,
) {
    if !render_scale.is_changed() {
        return;
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
        camera.target = RenderTarget::Image(new_image_handle.clone());
    }

    render_target_info.image_handle = new_image_handle;
}

/// Affiche dynamiquement la texture de rendu (RenderImage) via un sprite
pub fn display_render_image(
    mut commands: Commands,
    render_target: Res<RenderTargetInfo>,
    render_scale: Res<RenderScale>,
    windows: Query<&Window>,
    old_sprite_query: Query<Entity, With<RenderSprite>>,
) {
    if render_scale.is_changed() {
        for entity in old_sprite_query.iter() {
            commands.entity(entity).despawn();
        }

        let window = windows.single();
        let window_size = Vec2::new(window.resolution.width(), window.resolution.height());

        commands.spawn((
            SpriteBundle {
                texture: render_target.image_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(window_size),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            RenderSprite,
        ));
    }
}

/// Déplacement de la caméra avec les touches fléchées
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

/// Applique le zoom dynamique à la caméra 3D
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
