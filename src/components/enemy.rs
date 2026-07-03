//! This module defines core structures and setup behaviors for game enemies.

use crate::{GameState, Wall, check_collision};
use bevy::{color::palettes::css::DARK_MAGENTA, prelude::*};
use rand::random;

/// Component representing an enemy `Enemy`.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::enemy::Enemy;
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

/// The `movement speed` of the `enemies`.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::enemy::EnemySpeed;
///
/// App::new().insert_resource(EnemySpeed(100.0)).update();
/// ```
#[derive(Resource, Debug, Clone)]
pub struct EnemySpeed(pub f32);

impl Enemy {
    fn new(name: Name, speed: EnemySpeed, pos: Vec3) -> Self {
        Self { name, speed, pos }
    }
}

/// The [`Plugin`] for enemy systems.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::enemy::EnemyPlugin;
///
/// App::new().add_plugins(EnemyPlugin).update();
/// ```
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
/// use slots_from_hell::components::enemy::{Enemy, Name, add_enemies};
///
/// fn get_enemy_names(query: Query<&Name, With<Enemy>>) {
///     for name in &query {
///         println!("{}", name.0);
///     }
/// }
/// App::new()
///     .add_plugins(MeshPlugin)
///     .add_systems(Startup, add_enemies)
///     .add_systems(Update, get_enemy_names)
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
/// use slots_from_hell::components::enemy::{ add_enemies, Enemy};
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
            Mesh2d(meshes.add(Rectangle::new(64.0, 128.0))),
            MeshMaterial2d(materials.add(Color::from(DARK_MAGENTA))),
            Transform::from_translation(e.pos),
        ));
    });
    cmds.insert_resource(EnemySpeed(300.0));
}

/// The logic for moving the enemies.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::enemy::{EnemySpeed, enemy_movement};
///
/// let mut app = App::new();
/// app.insert_resource(EnemySpeed(100.0)).update();
/// app.add_systems(Update, enemy_movement).update();
/// ```
pub fn enemy_movement(
    enemies_query: Query<&mut Transform, With<Enemy>>,
    wall_query: Query<(&Transform, &Wall), Without<Enemy>>,
    speed: Res<EnemySpeed>,
    time: Res<Time<Fixed>>,
) {
    for mut enemies in enemies_query {
        let direction = Vec3::new(random::<f32>(), random::<f32>(), 0.0);

        let move_delta = direction.normalize_or_zero() * speed.0 * time.delta_secs();
        let new_pos = enemies.translation + move_delta;
        let enemy_size = Vec2::new(128.0, 64.0);

        let mut collision = false;
        for (wall_tf, wall) in &wall_query {
            if check_collision(new_pos, enemy_size, wall_tf, wall) {
                collision = true;
                break;
            }
        }
        if !collision {
            enemies.translation = new_pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::mesh::MeshPlugin;

    use super::*;

    /*#[test]
    fn test_add_enemies() {
        let mut app = App::new();

        app.add_plugins(MeshPlugin)
            .insert_resource()
            .add_systems(Startup, add_enemies)
            .update();

        let count = app
            .world_mut()
            .query_filtered::<(), With<Enemy>>()
            .iter(app.world())
            .count();
        assert_eq!(count, 4);
    }*/
}
