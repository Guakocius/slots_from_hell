use bevy::prelude::*;

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
