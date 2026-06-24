use bevy::prelude::*;

use slots_from_hell::components::{
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
