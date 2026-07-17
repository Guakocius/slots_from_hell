//! Module for the generation of the FPS overlay.
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
    text::FontSmoothing,
};

#[derive(Resource)]
struct FpsOverlayColor;

impl FpsOverlayColor {
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
                text_color: FpsOverlayColor::GREEN,
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    ..default()
                },
            },
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
