use bevy::prelude::*;

pub struct ClockPlugin;

macro_rules! spawn_text {
    ($cmds:expr, $(($t:expr,$pos1:ident,$pos2:ident,$x:expr,$y:expr)),* $(,)?) => {
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
            ));
        )*
    };
}

#[derive(Resource)]
struct ClockTimer(Timer);

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClockTimer(Timer::from_seconds(60.0, TimerMode::Repeating)))
            .add_systems(Startup, setup);
    }
}

fn setup(mut cmds: Commands) {
    let mut day = "Day 1";
    let mut time = "0 am";

    spawn_text!(cmds, (day, top, right, 12, 12), (time, top, right, 40, 12));
}
