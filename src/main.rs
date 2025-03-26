use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SliderState { is_dragging: false })
        .add_systems(Startup, setup)
        .add_systems(Update, slider_system)
        .run();
}

#[derive(Component)]
struct SliderHandle;

#[derive(Resource)]
struct SliderState {
    is_dragging: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Barre du slider
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(50.0),
            height: Val::Px(10.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(25.0),
            top: Val::Percent(50.0),
            ..default()
        },
        background_color: Color::GRAY.into(),
        ..default()
    })
    .with_children(|parent| {
        // Poignée du slider
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(20.0),
                    height: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0), // Initialement aligné à gauche de la barre
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            SliderHandle,
        ));
    });
}

fn slider_system(
    mut slider_query: Query<&mut Style, With<SliderHandle>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut slider_state: ResMut<SliderState>,
) {
    let window = windows.single();
    let cursor_pos = window.cursor_position();

    if let Some(cursor_pos) = cursor_pos {
        for mut style in slider_query.iter_mut() {
            let slider_x = if let Val::Px(x) = style.left { x } else { 0.0 };

            let slider_width = 20.0;
            let bar_width = window.width() * 0.5; // 50% de la largeur de la fenêtre
            let bar_x = window.width() * 0.25; // Barre commence à 25% de la largeur de la fenêtre

            let cursor_x = cursor_pos.x;

            let is_hovering = cursor_x > (bar_x + slider_x) - slider_width / 2.0
                && cursor_x < (bar_x + slider_x) + slider_width / 2.0;

            if is_hovering && mouse_input.just_pressed(MouseButton::Left) {
                slider_state.is_dragging = true;
            }

            if mouse_input.just_released(MouseButton::Left) {
                slider_state.is_dragging = false;
            }

            if slider_state.is_dragging {
                let new_x = cursor_x - bar_x;
                let clamped_x = new_x.clamp(0.0, bar_width - slider_width);
                style.left = Val::Px(clamped_x);
            }
        }
    }
}
