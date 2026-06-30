//! This module defines core structures and setup behaviors for game entities.

#[doc(inline)]
use bevy::{color::palettes::css::DARK_MAGENTA, prelude::*};

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
pub struct Enemy {
    name: Name,
    pos: Vec3,
}

impl Enemy {
    fn new(name: Name, pos: Vec3) -> Self {
        Self { name, pos }
    }
}

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
#[derive(Component, Clone, Debug)]
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
        Enemy::new(
            Name("Asmodeus".to_string()),
            Vec3::new(1024.0, -1024.0, 0.0),
        ),
        Enemy::new(
            Name("Beelzebub".to_string()),
            Vec3::new(-1024.0, -1024.0, 0.0),
        ),
        Enemy::new(
            Name("Poltergeist".to_string()),
            Vec3::new(1024.0, 1024.0, 0.0),
        ),
        Enemy::new(Name("Lucifer".to_string()), Vec3::new(-1024.0, 0.0, 0.0)),
    ]
    .iter()
    .for_each(|e| {
        cmds.spawn((
            e.name.clone(),
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(DARK_MAGENTA))),
            Transform::from_translation(e.pos),
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
