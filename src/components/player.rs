//! A module for specifying the player's core behavior.
use bevy::{color::palettes::css::NAVY, prelude::*};

use crate::{GameState, Name, menu::MenuState};

/// The speed of the player defined as a resource for re-using.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::player::PlayerSpeed;
///
/// App::new().insert_resource(PlayerSpeed(100.0));
/// ```
#[derive(Resource)]
pub struct PlayerSpeed(
    /// Player speed as a 32 bit floating point number.
    pub f32,
);

#[derive(Resource)]
struct CameraDecayRate(f32);

/// A component representing the core essence of the Player which is then
/// globally shared.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::player::{Player, set_player_name};
///
/// App::new().add_systems(Update, set_player_name).update();
/// ```
#[derive(Component, Debug, Clone)]
pub struct Player {
    /// The player's name.
    pub name: String,
    /// The player's speed.
    pub speed: f32,
    /// Player position on the map.
    pub pos: Vec<Vec2>,
}

impl Player {
    /// Create a new Player with `name` and `speed` as their specifications.
    ///
    /// # Examples
    ///
    /// ```
    /// use slots_from_hell::components::player::Player;
    ///
    /// let name = String::from("Player");
    /// let speed = 100.0;
    /// let player = Player::new(name, speed);
    ///
    /// assert_eq!(player.name, String::from("Player"));
    /// assert_eq!(player.speed, 100.0);
    /// ```
    pub fn new(name: String, speed: f32) -> Self {
        Self {
            name,
            speed,
            pos: Vec::new(),
        }
    }
}

/// Plugin for the systems used by the [`Player`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::player::PlayerPlugin;
///
/// App::new().add_plugins((DefaultPlugins, PlayerPlugin));
/// ```
#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(FixedUpdate, (move_player, update_camera));
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = Player::new("Player".to_string(), 300.0);

    cmds.spawn((
        Mesh2d(meshes.add(Capsule2d::new(10.0, 8.0))),
        MeshMaterial2d(materials.add(Color::from(NAVY))),
        player.clone(),
    ));

    cmds.spawn((
        Text::new("Use WASD to move."),
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(10),
            ..default()
        },
    ));

    cmds.insert_resource(PlayerSpeed(player.speed));
    cmds.insert_resource(CameraDecayRate(2.0));
}

/// Sets the [`Name`] of the [`Player`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::player::set_player_name;
///
/// App::new().add_systems(Update, set_player_name).update();
/// ```
pub fn set_player_name(mut query: Query<&mut Name, With<Player>>) {
    if let Some(mut name) = (&mut query).into_iter().next() {
        name.0 = "Player2".to_string();
    }
}

fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };
    let Ok(player_tf) = player_query.single() else {
        return;
    };

    let target = player_tf.translation;
    camera_tf.translation = camera_tf.translation.lerp(target, 0.1);

    camera_tf.translation.x = camera_tf.translation.x.round();
    camera_tf.translation.y = camera_tf.translation.y.round();
    /*let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, camera_decay_rate.0, time.delta_secs());*/
}

/// This functions adds event handlers to check for the player's input and moves
/// the [`Player`] on the key presses `Ẁ`, `S`, `A` and `D` accordingly.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::player::move_player;
///
/// App::new().add_systems(Update, move_player);
/// ```
pub fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    speed: Res<PlayerSpeed>,
    time: Res<Time<Fixed>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let move_delta = direction.normalize_or_zero() * speed.0 * time.delta_secs();
    player.translation += move_delta.extend(0.0);
}

/// Checks for player input and handles the game logic accordingly.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::player::player_input;
///
/// App::new().add_plugins(DefaultPlugins).add_systems(Update, player_input);
/// ```
pub fn player_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if kb_input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
        menu_state.set(MenuState::Pause);
    }
}
