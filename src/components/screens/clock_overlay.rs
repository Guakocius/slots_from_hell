use bevy::prelude::*;

use crate::{GameState, insert_resources, spawn_text};

pub struct ClockPlugin;

#[derive(Resource, Deref, DerefMut, Clone)]
struct ClockTimer(Timer);

#[derive(Resource)]
struct DayRes(u8);

#[derive(Resource)]
struct DayTimeRes(u8);

#[derive(Component)]
struct Day;

#[derive(Component)]
struct DayTime;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, advance_time.run_if(in_state(GameState::Playing)));
    }
}

fn setup(mut cmds: Commands) {
    spawn_text!(
        cmds,
        ("Day 1", (top, 12), (right, 12), Day),
        ("0 am", (top, 40), (right, 12), DayTime)
    );

    insert_resources!(
        cmds,
        DayRes(1),
        DayTimeRes(0),
        ClockTimer(Timer::from_seconds(60.0, TimerMode::Repeating))
    );
}

fn advance_time(
    time: Res<Time>,
    mut _cmds: Commands,
    mut timer: ResMut<ClockTimer>,
    mut day: ResMut<DayRes>,
    mut day_time: ResMut<DayTimeRes>,
    mut day_text: Query<&mut Text, (With<Day>, Without<DayTime>)>,
    mut day_time_text: Query<&mut Text, (With<DayTime>, Without<Day>)>,
) {
    if timer.tick(time.delta()).just_finished() {
        day_time.0 += 1;
        let Ok(mut day_text) = day_text.single_mut() else {
            return;
        };
        let Ok(mut day_time_text) = day_time_text.single_mut() else {
            return;
        };

        day_time_text.0 = format!("{} am", day_time.0);
        if day_time.0 == 6 {
            day.0 += 1;
            day_text.0 = format!("Day {}", day.0);
            day_time.0 = 0;
            day_time_text.0 = format!("{} am", day_time.0);
        }
    }
}
