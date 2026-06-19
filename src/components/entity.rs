use bevy::prelude::*;

use super::player::Player;

#[derive(Component)]
pub struct Entity;
#[derive(Component)]
pub struct Name(pub String);

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

pub fn greet_entities(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Entity>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub fn add_entities(mut cmds: Commands) {
    cmds.spawn((Player, Name("Bob Testrop".to_string())));
    ["Entity1", "Entity2", "Entity3", "Entity4"]
        .iter()
        .for_each(|a| {
            cmds.spawn((Entity, Name(a.to_string())));
        });
}
