use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;
use bevy_asset::RenderAssetUsages; // Correct import for RenderAssetUsages

// Taille de l'hexagone
const HEX_SIZE: f32 = 30.0;
// Ecart entre les hexagones
const HEX_SPACING: f32 = 1.0;
// Dimensions de la grille
const GRID_WIDTH: i32 = 15;
const GRID_HEIGHT: i32 = 15;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup) // Remplace add_startup_system par add_systems
        .run();
}

// Positionne un hexagone dans la grille en nid d'abeille
fn hex_position(q: i32, r: i32, size: f32) -> Vec2 {
    let spacing = size + HEX_SPACING;
    let x_offset = if r % 2 == 0 {
        0.0 // No shift for even rows
    } else {
        spacing * 3.0_f32.sqrt() / 2.0 // Shift odd rows by half a hex width
    };
    let x = spacing * (3.0_f32.sqrt() * q as f32) + x_offset;
    let y = spacing * (3.0 / 2.0) * r as f32;
    Vec2::new(x, y)
}

// Calcule les sommets d'un hexagone
fn hex_corner(center: Vec2, size: f32, i: usize) -> Vec2 {
    let angle = std::f32::consts::PI / 3.0 * i as f32 + std::f32::consts::PI / 6.0;
    Vec2::new(center.x + size * angle.cos(), center.y + size * angle.sin())
}

// Génération de la grille d'hexagones
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Ajout de la caméra orthographique
    commands.spawn(Camera2dBundle::default());

    // Calculate offset to center the grid
    let spacing = HEX_SIZE + HEX_SPACING;
    let offset_x = -(GRID_WIDTH as f32) * spacing * 3.0_f32.sqrt() / 2.0;
    let offset_y = -(GRID_HEIGHT as f32) * spacing * (3.0 / 2.0) / 2.0;
    let offset = Vec2::new(offset_x, offset_y);

    for q in 0..GRID_WIDTH {
        for r in 0..GRID_HEIGHT {
            let center = hex_position(q, r, HEX_SIZE) + offset;
            spawn_hexagon(&mut commands, &mut meshes, &mut materials, center);
        }
    }
}

// Création d'un hexagone divisé en 6 triangles
fn spawn_hexagon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    center: Vec2,
) {
    let hex_vertices: Vec<Vec2> = (0..6).map(|i| hex_corner(center, HEX_SIZE, i)).collect();

    for i in 0..6 {
        let triangle_vertices = vec![
            [center.x, center.y, 0.0], // Centre de l'hexagone
            [hex_vertices[i].x, hex_vertices[i].y, 0.0], // Sommet actuel
            [hex_vertices[(i + 1) % 6].x, hex_vertices[(i + 1) % 6].y, 0.0], // Sommet suivant
        ];

        let indices = vec![0, 1, 2];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, triangle_vertices);
        mesh.insert_indices(Indices::U32(indices));

        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(ColorMaterial::from(Color::srgb(0.2, 0.5, 0.8)));

        commands.spawn((
            Mesh2d::from(mesh_handle),
            MeshMaterial2d(material_handle),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            GlobalTransform::default(),
        ));
    }
}
