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
macro_rules! is_in_room {
    ($etf:expr,$r:expr) => {
        $etf.translation.x - 512.0 < $r.pos.x + 512.0
            && $etf.translation.x + 512.0 > $r.pos.x - 512.0
            && $etf.translation.y - 512.0 < $r.pos.y + 512.0
            && $etf.translation.y + 512.0 > $r.pos.y - 512.0
    };
}
