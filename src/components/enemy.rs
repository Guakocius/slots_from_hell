//! This module defines core structures and setup behaviors for game entities.

#[doc(inline)]
use bevy::{color::palettes::css::DARK_MAGENTA, prelude::*};
use chacha20::cipher::typenum::Square;

#[doc(no_inline)]
use super::player::Player;

/// Component representing an enemy `Enemy`.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::entity::Enemy;
///
/// let mut app = App::new();
/// // Gets all Enemies locations allocated inside this App's World
/// app.world().entities();
/// ```
#[derive(Component)]
pub struct Enemy;

/// Component representing the unique identifier name of an [`Enemy`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::entity::{Enemy, Name, add_enemies};
///
/// fn get_entity_names(query: Query<&Name, With<Enemy>>) {
///     for name in &query {
///         println!("{}", name.0);
///     }
/// }
/// let mut app = App::new()
///     .add_systems(Startup, add_enemies)
///     .add_systems(Update, get_entity_names)
///     .update();
/// ```
#[derive(Component)]
pub struct Name(
    /// The `Name String`.
    pub String,
);

/// Spawns a `Command` thread which adds all enemies to the [`World`]
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::entity::{ add_entities, Enemy};
/// let mut app = App::new();
///
/// app.add_systems(Startup, add_enemies).update();
/// let count = app
///     .world_mut()
///     .query_filtered::<(), With<Enemy>>()
///     .iter(app.world())
///     .count();
///
/// assert_eq!(count, 4);
/// ```
pub fn add_enemies(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    [
        ("Asmodeus", Vec3::new(100.0, 100.0, 0.0)),
        ("Beelzebub", Vec3::new(-100.0, -100.0, 0.0)),
        ("Poltergeist", Vec3::new(250.0, 250.0, 0.0)),
        ("Lucifer", Vec3::new(-250.0, -250.0, 0.0)),
    ]
    .iter()
    .for_each(|(e, v)| {
        cmds.spawn((
            Enemy,
            Name(e.to_string()),
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(DARK_MAGENTA))),
            Transform::from_translation(*v),
        ));
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_enemies() {
        let mut app = App::new();
        app.add_systems(Startup, add_enemies).update();

        let count = app
            .world_mut()
            .query_filtered::<(), With<Enemy>>()
            .iter(app.world())
            .count();
        assert_eq!(count, 4);
    }
}
