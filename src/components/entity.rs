use bevy::prelude::*;

use super::player::Player;

#[derive(Component)]
pub struct Entity;
#[derive(Component)]
pub struct Name(pub String);

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

pub fn add_entities(mut cmds: Commands) {
    cmds.spawn((Player, Name("Bob Testrop".to_string())));
    ["Asmodeus", "Entity2", "Entity3", "Entity4"]
        .iter()
        .for_each(|a| {
            cmds.spawn((Entity, Name(a.to_string())));
        });
}
