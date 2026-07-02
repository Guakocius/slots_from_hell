//! This module defines core structures and setup behaviors for game entities.

use crate::{GameState, menu::MenuState};
use bevy::{color::palettes::css::DARK_MAGENTA, prelude::*};
use rand::{prelude::*, random};

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
    speed: EnemySpeed,
    pos: Vec3,
}

#[derive(Resource, Debug, Clone)]
pub struct EnemySpeed(pub f32);

impl Enemy {
    fn new(name: Name, speed: EnemySpeed, pos: Vec3) -> Self {
        Self { name, speed, pos }
    }
}

#[derive(Debug)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpeed(300.0))
            .add_systems(OnEnter(GameState::Playing), add_enemies)
            .add_systems(FixedUpdate, enemy_movement);
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
            EnemySpeed(300.0),
            Vec3::new(1024.0, -1024.0, 0.0),
        ),
        Enemy::new(
            Name("Beelzebub".to_string()),
            EnemySpeed(300.0),
            Vec3::new(-1024.0, -1024.0, 0.0),
        ),
        Enemy::new(
            Name("Poltergeist".to_string()),
            EnemySpeed(300.0),
            Vec3::new(1024.0, 1024.0, 0.0),
        ),
        Enemy::new(
            Name("Lucifer".to_string()),
            EnemySpeed(300.0),
            Vec3::new(-1024.0, 0.0, 0.0),
        ),
    ]
    .iter()
    .for_each(|e| {
        cmds.spawn((
            Enemy::new(e.name.clone(), e.speed.clone(), e.pos),
            e.name.clone(),
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(DARK_MAGENTA))),
            Transform::from_translation(e.pos),
        ));
    });
    cmds.insert_resource(EnemySpeed(300.0));
}

pub fn enemy_movement(
    mut enemies_query: Query<&mut Transform, With<Enemy>>,
    speed: Res<EnemySpeed>,
    time: Res<Time<Fixed>>,
) {
    for mut enemies in enemies_query {
        let direction = Vec3::new(random::<f32>(), random::<f32>(), 0.0);

        let move_delta = direction.normalize_or_zero() * speed.0 * time.delta_secs();
        enemies.translation += move_delta;
    }
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
