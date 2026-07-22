//! This module defines core structures and setup behaviors for game enemies.

use crate::{GameState, Player, Room, Wall, check_collision};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_northstar::{pathfind, prelude::*};

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
    sprite_path: String,
    can_move_through_walls: bool,
    state: EnemyState,
}

#[derive(SystemParam)]
pub struct EnemyQueries<'w, 's> {
    enemies: Query<'w, 's, (&'static mut Transform, &'static mut Enemy)>,
    player: Query<'w, 's, (&'static Transform, &'static Player), Without<Enemy>>,
    walls: Query<'w, 's, (&'static Transform, &'static Wall), Without<Enemy>>,
    rooms: Query<
        'w,
        's,
        (&'static Transform, &'static Room),
        (Without<Player>, Without<Wall>, Without<Enemy>),
    >,
}

#[derive(SystemParam)]
pub struct EnemyResources<'w> {
    speed: Res<'w, EnemySpeed>,
    time: Res<'w, Time<Fixed>>,
    game_state: ResMut<'w, NextState<GameState>>,
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

#[derive(Debug, Clone, Copy)]
enum EnemyState {
    Patrolling,
    Chasing,
}

impl Enemy {
    fn new(
        name: Name,
        speed: EnemySpeed,
        pos: Vec3,
        sprite_path: String,
        can_move_through_walls: bool,
        state: EnemyState,
    ) -> Self {
        Self {
            name,
            speed,
            pos,
            sprite_path,
            can_move_through_walls,
            state,
        }
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
/// use slots_from_hell::components::enemy::{add_enemies, Enemy};
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
pub fn add_enemies(mut cmds: Commands, assets: Res<AssetServer>) {
    [
        Enemy::new(
            Name("Asmodeus".to_string()),
            EnemySpeed(300.0),
            Vec3::new(1024.0, -1024.0, 0.0),
            String::from("img/asmodeus.png"),
            false,
            EnemyState::Patrolling,
        ),
        Enemy::new(
            Name("Beelzebub".to_string()),
            EnemySpeed(300.0),
            Vec3::new(-1024.0, -1024.0, 0.0),
            String::from("img/beelzebub.png"),
            false,
            EnemyState::Patrolling,
        ),
        Enemy::new(
            Name("Poltergeist".to_string()),
            EnemySpeed(300.0),
            Vec3::new(1024.0, 1024.0, 0.0),
            String::from("img/poltergeist.png"),
            true,
            EnemyState::Patrolling,
        ),
        Enemy::new(
            Name("Lucifer".to_string()),
            EnemySpeed(300.0),
            Vec3::new(-1024.0, 0.0, 0.0),
            String::from("img/lucifer.png"),
            false,
            EnemyState::Patrolling,
        ),
    ]
    .iter()
    .for_each(|e| {
        cmds.spawn((
            Enemy::new(
                e.name.clone(),
                e.speed.clone(),
                e.pos,
                e.sprite_path.to_string(),
                e.can_move_through_walls,
                e.state,
            ),
            e.name.clone(),
            Sprite {
                image: assets.load(&e.sprite_path),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..default()
            },
            Transform::from_translation(e.pos),
        ));
    });
    cmds.insert_resource(EnemySpeed(300.0));
}

fn move_to_player(
    grid: Single<&mut CardinalGrid>,
    start: Query<&Transform, With<Enemy>>,
    goal: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let Ok(start) = start.single() else {
        return;
    };
    let Ok(goal) = goal.single() else { return };
    let mut grid = grid.into_inner();
    let start_tf = start.translation;
    let start = UVec3::new(start_tf.x as u32, start_tf.y as u32, start_tf.z as u32);
    let goal_tf = goal.translation;
    let goal = UVec3::new(goal_tf.x as u32, goal_tf.y as u32, goal_tf.z as u32);

    let path_args = pathfind::PathfindArgs::new(start, goal);
    let path = path_args.astar();
}

/*#[derive(Resource, Debug)]
struct EnemyMovementTimer(Timer);*/

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
pub fn enemy_movement(mut cmds: Commands, mut queries: EnemyQueries, mut res: EnemyResources) {
    let Ok((player_tf, player)) = queries.player.single() else {
        return;
    };

    let player_pos = player_tf.translation;

    for (mut enemy_tf, mut enemy) in &mut queries.enemies {
        match enemy.state {
            EnemyState::Patrolling => {
                let mut timer = Timer::from_seconds(10.0, TimerMode::Repeating);

                for (room_tf, room) in &queries.rooms {
                    if check_collision!(
                        enemy_tf.translation,
                        Vec2::new(512.0, 512.0),
                        room_tf.translation
                    ) && check_collision!(
                        player_tf.translation,
                        Vec2::new(512.0, 512.0),
                        room_tf.translation
                    ) {
                        enemy.state = EnemyState::Chasing;
                    }
                }

                if timer.just_finished() {}
            }
            EnemyState::Chasing => {
                let direction = (player_pos - enemy_tf.translation).normalize_or_zero();

                let move_delta = direction * res.speed.0 * res.time.delta_secs();
                let new_pos = enemy_tf.translation + move_delta;
                let enemy_size = Vec2::new(64.0, 128.0);

                let mut collision = false;
                for (wall_tf, wall) in &queries.walls {
                    if check_collision!(
                        new_pos,
                        enemy_size,
                        wall_tf.translation,
                        Vec2::new(wall.height, wall.width)
                    ) && !enemy.can_move_through_walls
                    {
                        collision = true;
                        break;
                    }
                }
                if !collision {
                    enemy_tf.translation = new_pos;
                }
                if check_collision!(new_pos, enemy_size, player_pos, player.size) {
                    res.game_state.set(GameState::Dead);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
