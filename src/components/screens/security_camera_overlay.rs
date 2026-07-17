use bevy::prelude::*;

use crate::Player;

pub struct SecurityCameraPlugin;

impl Plugin for SecurityCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, swap_cameras);
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn((
        Text::new(concat!(
            "Press 1 to toggle cam 01.\n",
            "Press 2 to toggle cam 02.\n",
            "Press 3 to toggle cam 03\n",
            "Press 4 to toggle cam 04.\n",
            "Press 5 to toggle cam off."
        )),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
}

#[derive(Resource, Deref, DerefMut)]
struct AltUpdate(pub Schedule);

fn swap_schedule(mut schedules: ResMut<Schedules>, mut alt: ResMut<AltUpdate>) {
    std::mem::swap(schedules.get_mut(Update).unwrap(), &mut *alt);
}

fn swap_cameras(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    app_query: &mut App,
) {
    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };

    if input.just_pressed(KeyCode::Digit1) {
        camera_tf.translation = Vec3::new(1024.0, -1024.0, 0.0);
        app.update();
    }

    if input.just_pressed(KeyCode::Digit2) {
        camera_tf.translation = Vec3::new(-1024.0, -1024.0, 0.0);
        app.update();
    }

    if input.just_pressed(KeyCode::Digit3) {
        camera_tf.translation = Vec3::new(1024.0, 1024.0, 0.0);
        app.update();
    }

    if input.just_pressed(KeyCode::Digit4) {
        camera_tf.translation = Vec3::new(-1024.0, 0.0, 0.0);
        app.update();
    }

    if input.just_pressed(KeyCode::Digit5) {
        let Ok(player_tf) = player_query.single() else {
            return;
        };
        camera_tf.translation = player_tf.translation;
        app.update();
    }
}
