//! Module for all macros.
use crate::*;

/// Spawns text using the specified parameters.
///
/// # Examples
///
/// ```
/// spawn_text!(cmds, "Hello World".into(), (top, 12), (bottom, 12));
/// ```
#[macro_export]
macro_rules! spawn_text {
    ($cmds:expr,$(($t:expr,($pos1:ident,$x:expr),($pos2:ident,$y:expr)$(,$misc:expr),*)),*) => {
        $(
            $cmds.spawn((
                Text::new($t),
                TextFont {
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    $pos1: px($x),
                    $pos2: px($y),
                    ..default()
                },
                $($misc,)*
            ));
        )*
    };
}

/// Utility macro for mass-inserting all specified resources in a loop.
///
/// # Examples
///
/// ```
/// #[derive(Resource)]
/// struct Resource1;
/// #[derive(Resource)]
/// struct Resource2;
/// #[derive(Resource)]
/// struct Resource3;
///
/// insert_resources!(Resource1);
/// insert_resources!(Resource1, Resource2, Resource3);
/// ```
#[macro_export]
macro_rules! insert_resources {
    ($cmds:expr,$($res:expr),*) => {
        $(
            $cmds.insert_resource($res);
        )*
    };
}

/// Generate rooms with the specified arguments and creates a [`Vec`] out of this loop.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// generate_rooms!(("Test Room", Vec3::new(0.0, 0.0, -200.0), "textures/map_texture_floor.png"));
/// generate_rooms!(("Test Room 1", Vec3::new(0.0, 0.0, -200.0), "textures/map_texture_floor.png"), ("Test Room 2", Vec3::new(1024.0, 1024.0, -200.0), "textures/map_texture_floor.png"));
/// ```
#[macro_export]
macro_rules! generate_rooms {
    ($(($name:expr,$coords:expr,$texture:expr)),*) => {
        vec![
            $(
                Room::new($name.into(),$coords,$texture.into())
            ),*
        ]

    }
}

/// Checks if the `Entity` (either [`Player`] or [`Enemy`]) collides with the coordinates of either
/// a [`Room`] or, if a wall size is specified, a [`Wall`].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// check_collision!(Vec2::new(0.0, 0.0), Vec2::new(20.0, 20.0), Vec2::new(1024.0, 1024.0));
/// check_collision!(Vec2::new(0.0, 0.0), Vec2::new(20.0, 20.0), Vec2::new(1024.0, 1024.0), Vec2::new(128.0, 64.0));
/// ```
#[macro_export]
macro_rules! check_collision {
    ($pos:expr,$sz:expr,$opos:expr) => {
        check_collision!($pos, $sz, $opos, Vec2::splat(512.0))
    };
    ($pos:expr,$sz:expr,$opos:expr,$osz:expr) => {{
        let pos_min = Vec2::new($pos.x - $sz.x / 2.0, $pos.y - $sz.y / 2.0);
        let pos_max = Vec2::new($pos.x + $sz.x / 2.0, $pos.y + $sz.y / 2.0);

        let min = $opos.truncate() - $osz / 2.0;
        let max = $opos.truncate() + $osz / 2.0;

        pos_min.x < max.x && pos_max.x > min.x && pos_min.y < max.y && pos_max.y > min.y
    }};
}
