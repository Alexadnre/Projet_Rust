use bevy::prelude::*;
use bevy::sprite::{Mesh2dHandle, MaterialMesh2dBundle};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::mesh::Indices;
use noise::{NoiseFn, Perlin};

// Constantes initiales
const GRID_WIDTH: i32 = 15;
const GRID_HEIGHT: i32 = 15;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SliderState { 
            is_dragging: false, 
            value: 10.0,
        })
        .insert_resource(RoadDensityState { 
            is_dragging: false, 
            value: 0.7, // Valeur initiale de ROAD_DENSITY
        })
        .insert_resource(TileSpacing { factor: 0.1 }) // 10% de la taille par défaut
        .insert_resource(RoadNoise { perlin: Perlin::new(42) }) // Seed fixe pour la reproductibilité
        .add_systems(Startup, setup)
        .add_systems(Update, (slider_system, update_hexagons))
        .run();
}

#[derive(Component)]
struct SliderHandle;

#[derive(Component)]
struct RoadDensitySliderHandle;

#[derive(Component)]
struct SliderText;

#[derive(Component)]
struct RoadDensityText;

#[derive(Component)]
struct Hexagon;

#[derive(Component)]
struct Road;

#[derive(Resource)]
struct SliderState {
    is_dragging: bool,
    value: f32,
}

#[derive(Resource)]
struct RoadDensityState {
    is_dragging: bool,
    value: f32,
}

#[derive(Resource)]
struct TileSpacing {
    factor: f32,
}

#[derive(Resource)]
struct RoadNoise {
    perlin: Perlin,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Slider pour la taille des hexagones
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Taille: 10",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(50.0),
                top: Val::Px(20.0),
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
            top: Val::Px(50.0),
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

    // Slider pour la densité des routes
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Densité routes: 0.7",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(50.0),
                top: Val::Px(80.0),
                ..default()
            },
            ..default()
        },
        RoadDensityText,
    ));

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(6.0),
            position_type: PositionType::Absolute,
            right: Val::Px(50.0),
            top: Val::Px(110.0),
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
                    left: Val::Px(84.0), // Position initiale pour 0.7 (70% de 150-12)
                    top: Val::Px(-9.0),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            RoadDensitySliderHandle,
        ));
    });

    spawn_hex_grid(&mut commands, 10.0, 0.1);
}

fn hex_position(q: i32, r: i32, size: f32, spacing_factor: f32) -> Vec2 {
    let spacing = size * spacing_factor;
    let total_size = size + spacing;
    let x = total_size * 3.0_f32.sqrt() * q as f32 
        + (if r % 2 == 1 { total_size * 3.0_f32.sqrt() / 2.0 } else { 0.0 });
    let y = total_size * 3.0 / 2.0 * r as f32;
    Vec2::new(x, y)
}

fn hex_corner(center: Vec2, size: f32, i: usize) -> Vec2 {
    let angle = std::f32::consts::PI / 3.0 * i as f32 + std::f32::consts::PI / 6.0;
    Vec2::new(center.x + size * angle.cos(), center.y + size * angle.sin())
}

fn spawn_hex_grid(commands: &mut Commands, hex_size: f32, spacing_factor: f32) {
    let spacing = hex_size * spacing_factor;
    let total_size = hex_size + spacing;
    let offset_x = -(GRID_WIDTH as f32 - 1.0) * total_size * 3.0_f32.sqrt() / 2.0;
    let offset_y = -(GRID_HEIGHT as f32 - 1.0) * total_size * 3.0 / 2.0 / 2.0;
    let offset = Vec2::new(offset_x, offset_y);

    for q in 0..GRID_WIDTH {
        for r in 0..GRID_HEIGHT {
            let center = hex_position(q, r, hex_size, spacing_factor) + offset;
            commands.spawn((Transform::from_translation(center.extend(0.0)), Hexagon));
        }
    }
}

fn slider_system(
    mut size_slider_query: Query<&mut Style, With<SliderHandle>>,
    mut density_slider_query: Query<&mut Style, (With<RoadDensitySliderHandle>, Without<SliderHandle>)>,
    mut size_text_query: Query<&mut Text, With<SliderText>>,
    mut density_text_query: Query<&mut Text, (With<RoadDensityText>, Without<SliderText>)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut size_slider_state: ResMut<SliderState>,
    mut density_slider_state: ResMut<RoadDensityState>,
) {
    let window = windows.single();
    let cursor_pos = window.cursor_position();

    if let Some(cursor_pos) = cursor_pos {
        let bar_width = 150.0;
        let bar_x = window.width() - 200.0;

        // Gestion du slider de taille
        for mut style in size_slider_query.iter_mut() {
            let slider_x = if let Val::Px(x) = style.left { x } else { 0.0 };

            let is_hovering = cursor_pos.x >= (bar_x + slider_x) - 6.0
                && cursor_pos.x <= (bar_x + slider_x) + 6.0
                && cursor_pos.y >= 50.0 - 10.0
                && cursor_pos.y <= 50.0 + 10.0;

            if is_hovering && mouse_input.just_pressed(MouseButton::Left) {
                size_slider_state.is_dragging = true;
            }

            if mouse_input.just_released(MouseButton::Left) {
                size_slider_state.is_dragging = false;
            }

            if size_slider_state.is_dragging {
                let new_x = cursor_pos.x - bar_x;
                let clamped_x = new_x.clamp(0.0, bar_width - 12.0);
                style.left = Val::Px(clamped_x);
                size_slider_state.value = 10.0 + (clamped_x / (bar_width - 12.0)) * 40.0;
            }
        }

        // Gestion du slider de densité des routes
        for mut style in density_slider_query.iter_mut() {
            let slider_x = if let Val::Px(x) = style.left { x } else { 0.0 };

            let is_hovering = cursor_pos.x >= (bar_x + slider_x) - 6.0
                && cursor_pos.x <= (bar_x + slider_x) + 6.0
                && cursor_pos.y >= 110.0 - 10.0
                && cursor_pos.y <= 110.0 + 10.0;

            if is_hovering && mouse_input.just_pressed(MouseButton::Left) {
                density_slider_state.is_dragging = true;
            }

            if mouse_input.just_released(MouseButton::Left) {
                density_slider_state.is_dragging = false;
            }

            if density_slider_state.is_dragging {
                let new_x = cursor_pos.x - bar_x;
                let clamped_x = new_x.clamp(0.0, bar_width - 12.0);
                style.left = Val::Px(clamped_x);
                density_slider_state.value = clamped_x / (bar_width - 12.0); // Valeur entre 0.0 et 1.0
            }
        }
    }

    // Mise à jour du texte pour la taille
    for mut text in size_text_query.iter_mut() {
        text.sections[0].value = format!("Taille: {:.0}", size_slider_state.value);
    }

    // Mise à jour du texte pour la densité
    for mut text in density_text_query.iter_mut() {
        text.sections[0].value = format!("Densité routes: {:.1}", density_slider_state.value);
    }
}

fn update_hexagons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hex_query: Query<(Entity, &Transform), With<Hexagon>>,
    road_query: Query<Entity, With<Road>>,
    slider_state: Res<SliderState>,
    density_state: Res<RoadDensityState>,
    tile_spacing: Res<TileSpacing>,
    road_noise: Res<RoadNoise>,
) {
    // Supprimer les anciens hexagones et routes
    for (entity, _) in hex_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in road_query.iter() {
        commands.entity(entity).despawn();
    }

    let hex_size = slider_state.value;
    let spacing = hex_size * tile_spacing.factor;
    let total_size = hex_size + spacing;
    let offset_x = -(GRID_WIDTH as f32 - 1.0) * total_size * 3.0_f32.sqrt() / 2.0;
    let offset_y = -(GRID_HEIGHT as f32 - 1.0) * total_size * 3.0 / 2.0 / 2.0;
    let offset = Vec2::new(offset_x, offset_y);

    // Générer les hexagones
    for q in 0..GRID_WIDTH {
        for r in 0..GRID_HEIGHT {
            let center = hex_position(q, r, hex_size, tile_spacing.factor) + offset;
            spawn_hexagon(&mut commands, &mut meshes, &mut materials, center, hex_size);
        }
    }

    // Générer les routes avec la densité dynamique
    spawn_roads(&mut commands, &mut meshes, &mut materials, hex_size, tile_spacing.factor, &road_noise, offset, density_state.value);
}

fn spawn_hexagon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    center: Vec2,
    size: f32,
) {
    let hex_vertices: Vec<Vec2> = (0..6).map(|i| hex_corner(center, size * 2.0, i)).collect();

    for i in 0..6 {
        let triangle_vertices = vec![
            [center.x, center.y, 0.0],
            [hex_vertices[i].x, hex_vertices[i].y, 0.0],
            [hex_vertices[(i + 1) % 6].x, hex_vertices[(i + 1) % 6].y, 0.0],
        ];

        let indices = vec![0, 1, 2];
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, triangle_vertices);
        mesh.insert_indices(Indices::U32(indices));

        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.5, 0.8)));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh_handle),
                material: material_handle,
                transform: Transform::from_translation(center.extend(0.0)),
                ..default()
            },
            Hexagon,
        ));
    }
}

fn spawn_roads(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    hex_size: f32,
    spacing_factor: f32,
    road_noise: &RoadNoise,
    offset: Vec2,
    road_density: f32, // Nouvelle densité dynamique
) {
    let mut generated_roads = std::collections::HashSet::new();
    let spacing = hex_size * spacing_factor;

    for q in 0..GRID_WIDTH {
        for r in 0..GRID_HEIGHT {
            let center = hex_position(q, r, hex_size, spacing_factor) + offset;
            let hex_vertices: Vec<Vec2> = (0..6).map(|i| hex_corner(center, hex_size, i)).collect();

            for i in 0..6 {
                let (dq, dr) = match i {
                    0 => (1, 0),   // Est
                    1 => (1, -1),  // Nord-Est
                    2 => (0, -1),  // Nord-Ouest
                    3 => (-1, 0),  // Ouest
                    4 => (-1, 1),  // Sud-Ouest
                    5 => (0, 1),   // Sud-Est
                    _ => unreachable!(),
                };

                let neighbor_q = q + dq;
                let neighbor_r = r + dr;

                if neighbor_q >= 0 && neighbor_q < GRID_WIDTH && neighbor_r >= 0 && neighbor_r < GRID_HEIGHT {
                    let start = hex_vertices[i];
                    let end = hex_vertices[(i + 1) % 6];

                    let road_id = if q <= neighbor_q && r <= neighbor_r {
                        (q, r, neighbor_q, neighbor_r)
                    } else {
                        (neighbor_q, neighbor_r, q, r)
                    };

                    if !generated_roads.contains(&road_id) {
                        generated_roads.insert(road_id);

                        // Utiliser les coordonnées de la grille (q, r) pour le bruit
                        let noise_value = road_noise.perlin.get([q as f64 * 0.1, r as f64 * 0.1]);
                        if noise_value > (1.0 - road_density as f64) {
                            spawn_road_segment(commands, meshes, materials, start * 2.0, end * 2.0, hex_size);
                        }
                    }
                }
            }
        }
    }
}

fn spawn_road_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start: Vec2,
    end: Vec2,
    hex_size: f32,
) {
    let road_thickness = hex_size * 0.5; // Épaisseur proportionnelle à la taille de l'hexagone
    let direction = (end - start).normalize();
    let perpendicular = Vec2::new(-direction.y, direction.x) * road_thickness / 2.0;

    // Créer un rectangle pour représenter le segment de route
    let vertices = vec![
        [start.x - perpendicular.x, start.y - perpendicular.y, 0.1],
        [start.x + perpendicular.x, start.y + perpendicular.y, 0.1],
        [end.x + perpendicular.x, end.y + perpendicular.y, 0.1],
        [end.x - perpendicular.x, end.y - perpendicular.y, 0.1],
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(Indices::U32(indices));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(ColorMaterial::from(Color::rgb(0.8, 0.8, 0.8))); // Couleur grise pour les routes

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh_handle),
            material: material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // Légèrement au-dessus des hexagones
            ..default()
        },
        Road,
    ));
}