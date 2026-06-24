//! This module defines core structures and setup behaviors for game entities.

#[doc(inline)]
use bevy::prelude::*;

#[doc(no_inline)]
use super::player::Player;

/// Component representing an enemy `Entity`.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::entity::Entity;
///
/// let mut app = App::new();
/// // Gets all Entities locations allocated inside this App's World
/// app.world().entities();
/// ```
#[derive(Component)]
pub struct Entity;

/// Component representing the unique identifier name of an [`Entity`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::entity::{Entity, Name, add_entities};
///
/// fn get_entity_names(query: Query<&Name, With<Entity>>) {
///     for name in &query {
///         println!("{}", name.0);
///     }
/// }
/// let mut app = App::new()
///     .add_systems(Startup, add_entities)
///     .add_systems(Update, get_entity_names)
///     .update();
/// ```
#[derive(Component)]
pub struct Name(
    /// The `Name String`.
    pub String,
);

/// Spawns a `Command` thread which adds the [`Player`] and all Entities to the [`World`]
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::entity::add_entities;
/// let mut app = App::new();
///
/// app.add_systems(Startup, add_entities).update();
/// let entities = app.world().entities().count_spawned();
///
/// assert_eq!(entities, 5);
/// ```
pub fn add_entities(mut cmds: Commands) {
    cmds.spawn((Player, Name("Player".to_string())));
    ["Asmodeus", "Entity2", "Entity3", "Entity4"]
        .iter()
        .for_each(|a| {
            cmds.spawn((Entity, Name(a.to_string())));
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entities() {
        let mut app = App::new();
        app.add_systems(Startup, add_entities).update();

        let entities = app.world().entities().count_spawned();
        println!("Entities: {:?}", entities);
        assert_eq!(entities, 5);
    }
}
