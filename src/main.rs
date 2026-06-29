use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};

use slots_from_hell::components::{
    enemy::add_enemies, player::PlayerPlugin, screens::game_menu::GameMenuPlugin,
    tilemap::TilemapPlugin,
};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_enemies);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Slots from Hell".into(),
                        present_mode: PresentMode::Mailbox,
                        ..default()
                    }),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin::default(),
            PlayerPlugin,
            TilemapPlugin,
            GameMenuPlugin,
            StartPlugin,
        ))
        .run();
}
