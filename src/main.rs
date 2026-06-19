use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct Animatronic;

fn add_entities(mut cmds: Commands) {
    cmds.spawn((Player, Name("Bob Testrop".to_string())));
    ["Bonnie", "Chica", "Freddy", "Foxy"].iter().for_each(|a| {
        cmds.spawn((Animatronic, Name(a.to_string())));
    });
}

fn greet_player(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn greet_animatronics(query: Query<&Name, With<Animatronic>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_player(mut query: Query<&mut Name, With<Player>>) {
    for mut name in &mut query {
        if name.0 == "Bob Testrop" {
            name.0 = "Bob Freddyson".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, add_entities)
            .add_systems(
                Update,
                (
                    hello_world,
                    (update_player, greet_player).chain(),
                    greet_animatronics,
                )
            );
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
