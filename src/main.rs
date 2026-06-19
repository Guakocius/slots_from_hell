use bevy::prelude::*;

use horror_game_juniper_game_jam::components::{
    entity::{GreetTimer, add_entities, greet_entities},
    player::update_player,
};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_entities)
            .add_systems(Update, (update_player, greet_entities));
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, StartPlugin)).run();
}
