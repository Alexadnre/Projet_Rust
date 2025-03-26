use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SliderState { is_dragging: false, value: 0.0 })
        .add_systems(Startup, setup)
        .add_systems(Update, slider_system)
        .run();
}

#[derive(Component)]
struct SliderHandle;

#[derive(Component)]
struct SliderText;

#[derive(Resource)]
struct SliderState {
    is_dragging: bool,
    value: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Texte affichant la valeur du slider
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Valeur: 0", 
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

    // Barre du slider
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(150.0), // Taille du slider réduite
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
        // Poignée du slider
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(12.0), // Poignée plus petite
                    height: Val::Px(24.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(-9.0), // Ajustement vertical
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
    mut text_query: Query<&mut Text, With<SliderText>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut slider_state: ResMut<SliderState>,
) {
    let window = windows.single();
    let cursor_pos = window.cursor_position();

    if let Some(cursor_pos) = cursor_pos {
        let bar_width = 150.0; // Largeur du slider en pixels
        let bar_x = window.width() - 200.0; // Position du slider (50px de marge)

        for mut style in slider_query.iter_mut() {
            let slider_x = if let Val::Px(x) = style.left { x } else { 0.0 };

            let is_hovering = cursor_pos.x >= (bar_x + slider_x) - 6.0
                && cursor_pos.x <= (bar_x + slider_x) + 6.0
                && cursor_pos.y >= 50.0 - 10.0
                && cursor_pos.y <= 50.0 + 10.0;

            if is_hovering && mouse_input.just_pressed(MouseButton::Left) {
                slider_state.is_dragging = true;
            }

            if mouse_input.just_released(MouseButton::Left) {
                slider_state.is_dragging = false;
            }

            if slider_state.is_dragging {
                let new_x = cursor_pos.x - bar_x;
                let clamped_x = new_x.clamp(0.0, bar_width - 12.0);
                style.left = Val::Px(clamped_x);

                // Mise à jour de la valeur du slider (0 à 100)
                slider_state.value = (clamped_x / (bar_width - 12.0)) * 100.0;
            }
        }
    }

    // Mise à jour du texte
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Valeur: {:.0}", slider_state.value);
    }
}
