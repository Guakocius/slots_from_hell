use bevy::prelude::*;

use horror_game_juniper_game_jam::components::{
    entity::{GreetTimer, add_entities},
    player::SetupPlugin,
    screens::game_menu::GameMenuPlugin,
};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_entities);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameMenuPlugin, StartPlugin, SetupPlugin))
        .run();
}
