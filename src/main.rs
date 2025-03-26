use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology, RenderAssetUsages};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

const HEX_SIZE: f32 = 30.0;
const HEX_SPACING: f32 = 1.0;
const MIN_GRID_SIZE: i32 = 5;
const MAX_GRID_SIZE: i32 = 20;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GridSize { width: 10, height: 10 })
        .insert_resource(SliderState { is_dragging: false })
        .add_systems(Startup, setup)
        .add_systems(Update, slider_system)
        .add_systems(Update, update_grid)
        .run();
}

#[derive(Resource)]
struct GridSize {
    width: i32,
    height: i32,
}

#[derive(Component)]
struct SliderHandle;

#[derive(Resource)]
struct SliderState {
    is_dragging: bool,
}

#[derive(Component)]
struct Grid;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_size: Res<GridSize>,
) {
    commands.spawn(Camera2dBundle::default());
    spawn_grid(&mut commands, &mut meshes, &mut materials, grid_size.width, grid_size.height);
}

fn spawn_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    width: i32,
    height: i32,
) {
    commands.spawn((Grid,));
    let spacing = HEX_SIZE + HEX_SPACING;
    let offset_x = -(width as f32) * spacing * 3.0_f32.sqrt() / 2.0;
    let offset_y = -(height as f32) * spacing * (3.0 / 2.0) / 2.0;
    let offset = Vec2::new(offset_x, offset_y);

    for q in 0..width {
        for r in 0..height {
            let center = hex_position(q, r, HEX_SIZE) + offset;
            spawn_hexagon(commands, meshes, materials, center);
        }
    }
}

fn update_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_size: Res<GridSize>,
    existing_grid: Query<Entity, With<Grid>>,
) {
    if grid_size.is_changed() {
        for entity in existing_grid.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_grid(&mut commands, &mut meshes, &mut materials, grid_size.width, grid_size.height);
    }
}

fn hex_position(q: i32, r: i32, size: f32) -> Vec2 {
    let spacing = size + HEX_SPACING;
    let x_offset = if r % 2 == 0 { 0.0 } else { spacing * 3.0_f32.sqrt() / 2.0 };
    let x = spacing * (3.0_f32.sqrt() * q as f32) + x_offset;
    let y = spacing * (3.0 / 2.0) * r as f32;
    Vec2::new(x, y)
}

fn hex_corner(center: Vec2, size: f32, i: usize) -> Vec2 {
    let angle = std::f32::consts::PI / 3.0 * i as f32 + std::f32::consts::PI / 6.0;
    Vec2::new(center.x + size * angle.cos(), center.y + size * angle.sin())
}

fn spawn_hexagon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    center: Vec2,
) {
    let hex_vertices: Vec<[f32; 3]> = (0..6)
        .map(|i| {
            let v = hex_corner(center, HEX_SIZE, i);
            [v.x, v.y, 0.0]
        })
        .collect();

    let mut indices = Vec::new();
    for i in 0..6 {
        indices.push(0);
        indices.push(i as u32);
        indices.push(((i + 1) % 6) as u32);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::empty());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, hex_vertices);
    mesh.insert_indices(Indices::U32(indices));
    
    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.5, 0.8)));

    commands.spawn(
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh_handle),
            material: material_handle,
            transform: Transform::from_translation(Vec3::new(center.x, center.y, 0.0)),
            ..default()
        },
    );
}

fn slider_system(
    mut slider_state: ResMut<SliderState>,
    mut mouse_input: ResMut<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        slider_state.is_dragging = true;
    }
    if mouse_input.just_released(MouseButton::Left) {
        slider_state.is_dragging = false;
    }
}
