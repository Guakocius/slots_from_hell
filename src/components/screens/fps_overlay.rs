//! Module for the generation of the FPS overlay.
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
    text::FontSmoothing,
};

#[derive(Resource)]
struct OverlayColor;

impl OverlayColor {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

/// A plugin configuring the FPS overlay.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::screens::fps_overlay::FpsPlugin;
///
/// App::new().add_plugins((DefaultPlugins, FpsPlugin));
/// ```
pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: FontSize::Px(20.0),
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                text_color: OverlayColor::GREEN,
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    ..default()
                },
            },
        })
        .add_systems(Startup, setup)
        .add_systems(Update, customize_config);
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn((
        Text::new(concat!(
            "Press 1 to toggle the overlay color.\n",
            "Press 2 to decrease the overlay size.\n",
            "Press 3 to increase the overlay size.\n",
            "Press 4 to toggle the text visibility.\n",
        )),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn customize_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::Digit1) {
        if overlay.text_color == OverlayColor::GREEN {
            overlay.text_color = OverlayColor::RED;
        } else {
            overlay.text_color = OverlayColor::GREEN;
        }
    }

    if let FontSize::Px(mut px_font_size) = overlay.text_config.font_size {
        if input.just_pressed(KeyCode::Digit2) {
            px_font_size = (px_font_size - 2.0).max(2.0);
        }
        if input.just_pressed(KeyCode::Digit3) {
            px_font_size += 2.0;
        }
        if FontSize::Px(px_font_size) != overlay.text_config.font_size {
            overlay.text_config.font_size = FontSize::Px(px_font_size);
        }
    }

    if input.just_pressed(KeyCode::Digit4) {
        overlay.enabled = !overlay.enabled;
    }
}
