use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use noise::{NoiseFn, Perlin}; // Import the Perlin noise generator

#[derive(Component)]
struct Hexagon {
    q: i32,
    r: i32,
    radius: f32,
}

#[derive(Component)]
struct TriangleVertex;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Point Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0),
        ..default()
    });

    // Directional Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4, 0.0)),
        ..default()
    });

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0, subdivisions: 0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.2),
            perceptual_roughness: 0.8,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, -0.1, 0.0)),
        ..default()
    });

    // Initialize Perlin noise generator
    let perlin = Perlin::new(42); // Use a fixed seed for reproducibility

    // Hexagonal Grid Parameters
    let hex_radius = 1.0;
    let grid_size = 10;

    let mut red_points = Vec::new();

    // Collect all red points (triangle vertices)
    for q in -grid_size..=grid_size {
        for r in -grid_size..=grid_size {
            if q + r > grid_size || q + r < -grid_size {
                continue;
            }

            let position = hex_to_world(q, r, hex_radius);
            let rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_6);
            let vertex_positions = get_triangle_vertices(hex_radius);

            for vertex_pos in vertex_positions {
                let transformed_vertex = position + rotation.mul_vec3(vertex_pos);
                red_points.push(transformed_vertex);

                // // Spawn red points
                // commands.spawn((
                //     PointLightBundle {
                //         point_light: PointLight {
                //             intensity: 1.0,
                //             color: Color::rgb(1.0, 0.0, 0.0),
                //             range: 5.0,
                //             ..default()
                //         },
                //         transform: Transform::from_translation(transformed_vertex),
                //         ..default()
                //     },
                //     TriangleVertex
                // ));
            }
        }
    }

    // Connect red points to their nearest neighbors
    for i in 0..red_points.len() {
        let start = red_points[i];
        let mut closest_distance = f32::MAX;
        let mut closest_point = None;

        for j in 0..red_points.len() {
            if i == j {
                continue;
            }

            let end = red_points[j];
            let distance = (end - start).length();

            if distance < closest_distance {
                closest_distance = distance;
                closest_point = Some(end);
            }
        }

        if let Some(end) = closest_point {
            let direction = (end - start).normalize();
            let length = (end - start).length();
            let road_width = 0.1;

            // Spawn road as a thick line
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -road_width / 2.0,
                    max_x: road_width / 2.0,
                    min_y: -0.01,
                    max_y: 0.01,
                    min_z: 0.0,
                    max_z: length,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.5, 0.5, 0.5),
                    perceptual_roughness: 0.9,
                    ..default()
                }),
                transform: Transform {
                    translation: start + direction * (length / 2.0),
                    rotation: Quat::from_rotation_arc(Vec3::Z, direction),
                    ..default()
                },
                ..default()
            });
        }
    }

    // Generate hexagonal terrain with roads on edges
    for q in -grid_size..=grid_size {
        for r in -grid_size..=grid_size {
            if q + r > grid_size || q + r < -grid_size {
                continue;
            }
            
            let position = hex_to_world(q, r, hex_radius);
            let rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_6);
            
            // Spawn Hexagon Mesh
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(create_hex_mesh(hex_radius)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.3, 0.5, 0.3),
                        perceptual_roughness: 0.6,
                        ..default()
                    }),
                    transform: Transform {
                        translation: position,
                        rotation,
                        ..default()
                    },
                    ..default()
                },
                Hexagon { q, r, radius: hex_radius }
            ));

            // Generate roads between red points based on Perlin noise
            let vertex_positions = get_triangle_vertices(hex_radius);
            for i in 0..vertex_positions.len() {
                let start = position + rotation.mul_vec3(vertex_positions[i]);
                let end = position + rotation.mul_vec3(vertex_positions[(i + 1) % vertex_positions.len()]);
                let midpoint = (start + end) / 2.0;

                // Use Perlin noise to decide if a road should be placed
                let noise_value = perlin.get([midpoint.x as f64, midpoint.z as f64]);
                if noise_value > 0.0 { // Example threshold for road placement
                    let direction = (end - start).normalize();
                    let length = (end - start).length();
                    let road_width = 0.1; // Adjust width for thicker lines

                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box {
                            min_x: -road_width / 2.0,
                            max_x: road_width / 2.0,
                            min_y: -0.01, // Slightly above ground
                            max_y: 0.01,
                            min_z: 0.0,
                            max_z: length,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.5, 0.5, 0.5), // Gray for roads
                            perceptual_roughness: 0.9,
                            ..default()
                        }),
                        transform: Transform {
                            translation: start + direction,
                            rotation: Quat::from_rotation_arc(Vec3::Z, direction),
                            ..default()
                        },
                        ..default()
                    });
                }
            }

            // Spawn Wireframe
            commands.spawn(PbrBundle {
                mesh: meshes.add(create_hex_wireframe(hex_radius)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    unlit: true,
                    ..default()
                }),
                transform: Transform {
                    translation: position,
                    rotation,
                    ..default()
                },
                ..default()
            });

            // Spawn Triangle Vertices
            let vertex_positions = get_triangle_vertices(hex_radius);
            for vertex_pos in vertex_positions {
                let transformed_vertex = position + rotation.mul_vec3(vertex_pos);
                
                // commands.spawn((
                //     PointLightBundle {
                //         point_light: PointLight {
                //             intensity: 0.0,  // Reduced intensity
                //             color: Color::rgb(1.0, 1.0, 0.0),
                //             range: 5.0,  // Reduced range
                //             ..default()
                //         },
                //         transform: Transform::from_translation(transformed_vertex),
                //         ..default()
                //     },
                //     TriangleVertex
                // ));
            }
        }
    }
}

// Convert hex grid coordinates to world space
fn hex_to_world(q: i32, r: i32, radius: f32) -> Vec3 {
    let x = radius * 3.0f32.sqrt() * (q as f32 + r as f32 / 2.0);
    let z = radius * 3.0 / 2.0 * r as f32;
    Vec3::new(x, 0.0, z)
}

// Get triangle vertex positions
fn get_triangle_vertices(radius: f32) -> Vec<Vec3> {
    let mut vertices = Vec::new();
    for i in 0..6 {
        let angle = std::f32::consts::PI / 3.0 * i as f32;
        vertices.push(Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin()));
    }
    vertices
}

// Create a filled hexagon mesh
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

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; positions.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
    mesh
}

// Create a wireframe for the hexagon
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

// Create a line mesh between two points
fn create_line_mesh(start: Vec3, end: Vec3) -> Mesh {
    let positions = vec![
        [start.x, start.y, start.z],
        [end.x, end.y, end.z],
    ];
    let indices = vec![0, 1];

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}