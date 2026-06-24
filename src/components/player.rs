//! A module for specifying the player's core behavior.
use bevy::{post_process::bloom::Bloom, prelude::*};

use crate::{
    GameState, Name,
    menu::{MenuButtonAction, MenuState},
};

const PLAYER_SPEED: f32 = 100.0;
const CAMERA_DECAY_RATE: f32 = 2.0;

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
#[derive(Component)]
pub struct Player;

/// Generates a UI Text with the move controls the [`Player`] has.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::player::setup_instructions;
///
/// App::new().add_systems(Startup, setup_instructions).update();
/// ```
pub fn setup_instructions(mut cmds: Commands) {
    cmds.spawn((
        Text::new("Move the flashlight with WASD."),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
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

/// This functions updates the `Position` of the [`Camera`] by aligning it to the
/// `Player's` coordinates.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::player::update_camera;
///
/// App::new().add_systems(Update, update_camera).update();
/// ```
pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
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
/// App::new().add_systems(Update, move_player).update();
/// ```
pub fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
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

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
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
        game_state.set(GameState::Menu);
        menu_state.set(MenuState::Main);
    }
}
