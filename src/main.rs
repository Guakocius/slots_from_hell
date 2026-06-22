use bevy::prelude::*;

use horror_game_juniper_game_jam::components::{
    entity::add_entities, screens::game_menu::GameMenuPlugin, tilemap::TilemapPlugin,
};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_entities);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TilemapPlugin,
            GameMenuPlugin,
            StartPlugin,
        ))
        .run();
}
