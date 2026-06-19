use bevy::prelude::*;

use super::entity::Name;

#[derive(Component)]
pub struct Player;

pub fn update_player(mut query: Query<&mut Name, With<Player>>) {
    for mut name in &mut query {
        if name.0 == "Bob Testrop" {
            name.0 = "Bob Freddyson".to_string();
            break;
        }
    }
}
