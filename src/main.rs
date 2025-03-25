use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup) // Corrected to use `add_startup_system`
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y), // Adjusted position
        ..default()
    });

    // Point Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 8000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0),
        ..default()
    });

    // Directional Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0, // Stronger light for better visibility
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4, 0.0)), // Angled light
        ..default()
    });

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0, subdivisions: 0 })), // Added `subdivisions`
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.2),
            perceptual_roughness: 0.8, // Add roughness for realism
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Slightly below hexagons
        ..default()
    });

    // Generate hexagonal terrain with wireframe
    let hex_radius = 1.0;
    let grid_size = 10;
    for q in -grid_size..=grid_size {
        for r in -grid_size..=grid_size {
            if q + r > grid_size || q + r < -grid_size {
                continue;
            }
            let position = hex_to_world(q, r, hex_radius);
            let mesh = create_hex_mesh(hex_radius);
            commands.spawn(PbrBundle {
                mesh: meshes.add(mesh.clone()),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.3, 0.5, 0.3),
                    perceptual_roughness: 0.6,
                    ..default()
                }),
                transform: Transform::from_translation(position),
                ..default()
            });

            // Add wireframe as a separate mesh
            let wireframe_mesh = create_hex_wireframe(hex_radius);
            commands.spawn(PbrBundle {
                mesh: meshes.add(wireframe_mesh),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(position),
                ..default()
            });

            // Add a red point at the center of each hexagon
            commands.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 50.0, // Adjust intensity for visibility
                    color: Color::rgb(1.0, 0.0, 0.0), // Red color
                    range: 1.0, // Small range to act as a point
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            });
        }
    }
}

#[derive(Component)]
struct Hexagon {
    radius: f32,
}

fn hex_to_world(q: i32, r: i32, radius: f32) -> Vec3 {
    // Calcul de la position en X et Z avec espacement ajusté
    let x = radius * 3.0f32.sqrt() * (q as f32 + r as f32 / 2.0);  // Espacement sur l'axe X
    let z = radius * 3.0 / 2.0 * r as f32;  // Espacement sur l'axe Z

    Vec3::new(x, 0.0, z)
}


fn create_hex_mesh(radius: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    // Center of the hexagon
    positions.push([0.0, 0.0, 0.0]);

    // Vertices of the hexagon
    for i in 0..6 {
        let angle = std::f32::consts::PI / 3.0 * i as f32;
        positions.push([radius * angle.cos(), 0.0, radius * angle.sin()]);
    }

    // Triangles (center to vertices)
    for i in 0..6 {
        indices.push(0);
        indices.push((i + 1) as u32);
        indices.push(((i + 1) % 6 + 1) as u32);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList); // Use only one argument
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone()); // Clone positions
    mesh.set_indices(Some(Indices::U32(indices))); // Use set_indices
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; positions.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
    mesh
}

fn create_hex_wireframe(radius: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    // Vertices of the hexagon
    for i in 0..6 {
        let angle = std::f32::consts::PI / 3.0 * i as f32;
        positions.push([radius * angle.cos(), 0.0, radius * angle.sin()]);
    }

    // Edges of the hexagon
    for i in 0..6 {
        indices.push(i as u32);
        indices.push(((i + 1) % 6) as u32);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
