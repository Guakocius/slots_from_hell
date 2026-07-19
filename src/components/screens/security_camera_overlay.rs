use bevy::prelude::*;

use crate::{Player, spawn_text};

pub struct SecurityCameraPlugin;

#[derive(Resource)]
pub struct CameraSwitch;

impl Plugin for SecurityCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, swap_cameras);
    }
}

fn setup(mut cmds: Commands) {
    spawn_text!(
        cmds,
        (
            concat!(
                "Press 1 to toggle cam 01.\n",
                "Press 2 to toggle cam 02.\n",
                "Press 3 to toggle cam 03\n",
                "Press 4 to toggle cam 04.\n",
                "Press 5 to toggle cam off."
            ),
            (bottom, 12),
            (left, 12)
        )
    );
}

fn swap_cameras(
    mut cmds: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    camera_res: Option<Res<CameraSwitch>>,
) {
    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };

    if input.just_pressed(KeyCode::Digit1) {
        camera_tf.translation = Vec3::new(1024.0, -1024.0, 0.0);
        if camera_res.is_none() {
            cmds.insert_resource(CameraSwitch);
        }
    }

    if input.just_pressed(KeyCode::Digit2) {
        camera_tf.translation = Vec3::new(-1024.0, -1024.0, 0.0);
        if camera_res.is_none() {
            cmds.insert_resource(CameraSwitch);
        }
    }

    if input.just_pressed(KeyCode::Digit3) {
        camera_tf.translation = Vec3::new(1024.0, 1024.0, 0.0);
        if camera_res.is_none() {
            cmds.insert_resource(CameraSwitch);
        }
    }

    if input.just_pressed(KeyCode::Digit4) {
        camera_tf.translation = Vec3::new(-1024.0, 0.0, 0.0);
        if camera_res.is_none() {
            cmds.insert_resource(CameraSwitch);
        }
    }

    if input.just_pressed(KeyCode::Digit5) {
        let Ok(player_tf) = player_query.single() else {
            return;
        };
        camera_tf.translation = player_tf.translation;
        if camera_res.is_some() {
            cmds.remove_resource::<CameraSwitch>();
        }
    }
}
