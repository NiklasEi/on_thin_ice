use crate::GameState;
use bevy::prelude::*;

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Countdown).with_system(start_timer))
            .add_system_set(SystemSet::on_update(GameState::Countdown).with_system(countdown));
    }
}

fn start_timer(mut commands: Commands) {
    commands.insert_resource(CountdownTimer::default());
}

struct CountdownTimer(Timer);

impl Default for CountdownTimer {
    fn default() -> Self {
        CountdownTimer(Timer::from_seconds(3., false))
    }
}

fn countdown(
    time: Res<Time>,
    mut timer: ResMut<CountdownTimer>,
    mut state: ResMut<State<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        state.set(GameState::Playing).expect("Failed to set state");
    }
}
