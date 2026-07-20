use bevy::prelude::*;

use crate::Room;

#[macro_export]
macro_rules! spawn_text {
    ($cmds:expr,$(($t:expr,($pos1:ident,$x:expr),($pos2:ident,$y:expr)$(,$misc:expr),*)),*) => {
        $(
            $cmds.spawn((
                Text::new($t),
                TextFont {
                    font_size: FontSize::Px(15.0),
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

#[macro_export]
macro_rules! insert_resources {
    ($cmds:expr,$($res:expr),*) => {
        $(
            $cmds.insert_resource($res);
        )*
    };
}

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
