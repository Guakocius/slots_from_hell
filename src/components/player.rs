//! A module for specifying the player's core behavior.
use bevy::{color::palettes::css::NAVY, prelude::*};

use crate::{CameraSwitch, GameState, Name, Wall, check_collision, menu::MenuState};

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
#[derive(Resource, Debug, Clone)]
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
/// use slots_from_hell::components::player::{Player, PlayerSpeed};
///
/// let player = Player::new("John Doe".into(), PlayerSpeed(100.0), Vec3::new(0.0, 0.0, 0.0));
///
/// assert_eq!(player.name, "John Doe".to_string());
/// assert_eq!(player.speed.0, 100.0);
/// ```
#[derive(Component, Debug, Clone)]
pub struct Player {
    /// The player's name.
    pub name: String,
    /// The player's speed.
    pub speed: PlayerSpeed,
    /// Player position on the map.
    pub pos: Vec3,
}

impl Player {
    /// Create a new Player with `name` and `speed` as their specifications.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use slots_from_hell::components::player::{Player, PlayerSpeed};
    ///
    /// let name = String::from("Player");
    /// let speed = 100.0;
    /// let player = Player::new(name, PlayerSpeed(speed), Vec3::new(0.0, 0.0, 0.0));
    ///
    /// assert_eq!(player.name, String::from("Player"));
    /// assert_eq!(player.speed.0, 100.0);
    /// ```
    pub fn new(name: String, speed: PlayerSpeed, pos: Vec3) -> Self {
        Self { name, speed, pos }
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
        app.insert_resource(PlayerSpeed(300.0))
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(FixedUpdate, (move_player, update_camera));
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = Player::new(
        "Player".to_string(),
        PlayerSpeed(300.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

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
            ..default()
        },
    ));

    cmds.insert_resource(PlayerSpeed(player.speed.0));
    cmds.insert_resource(CameraDecayRate(2.0));
}

fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    camera_switch: Option<Res<CameraSwitch>>,
) {
    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };
    let Ok(player_tf) = player_query.single() else {
        return;
    };

    if camera_switch.is_some() {
        return;
    }
    let target = player_tf.translation;
    camera_tf.translation = camera_tf.translation.lerp(target, 0.1);

    camera_tf.translation.x = camera_tf.translation.x.round();
    camera_tf.translation.y = camera_tf.translation.y.round();
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
    mut player_query: Query<&mut Transform, With<Player>>,
    wall_query: Query<(&Transform, &Wall), Without<Player>>,
    speed: Res<PlayerSpeed>,
    time: Res<Time<Fixed>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    camera_switch: Option<Res<CameraSwitch>>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if camera_switch.is_some() {
        return;
    }

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

    let new_pos = transform.translation + move_delta.extend(0.0);
    let player_size = Vec2::new(20.0, 20.0);

    let mut collision = false;
    for (wall_tf, wall) in &wall_query {
        if check_collision!(
            new_pos,
            player_size,
            wall_tf.translation,
            Vec2::new(wall.height, wall.width)
        ) {
            collision = true;
            break;
        }
    }
    if !collision {
        transform.translation = new_pos;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_new() {
        let player = Player::new("Testname".into(), PlayerSpeed(100.0), Vec3::ZERO);

        assert_eq!(player.name, String::from("Testname"));
        assert_eq!(player.speed.0, 100.0);
    }

    #[test]
    fn test_player_plugin_build() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::state::app::StatesPlugin,
            AssetPlugin::default(),
            PlayerPlugin,
        ))
        .init_asset::<Mesh>()
        .init_asset::<ColorMaterial>()
        .init_state::<GameState>();
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();

        let player_count = app.world_mut().query::<&Player>().iter(app.world()).count();

        assert!(app.is_plugin_added::<PlayerPlugin>());
        assert_eq!(player_count, 1);
    }

    #[test]
    fn test_update_camera() {}

    #[test]
    fn test_player_movement() {}

    #[test]
    fn test_player_input() {}
}
