use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};

use slots_from_hell::components::{
    enemy::EnemyPlugin, player::PlayerPlugin, screens::game_menu::GameMenuPlugin,
    tilemap::TilemapPlugin,
};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
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
            GameMenuPlugin,
            PlayerPlugin,
            EnemyPlugin,
            TilemapPlugin,
        ));
    }
}

fn main() {
    App::new().add_plugins(StartPlugin).run();
}
