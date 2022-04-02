use crate::ice::IceLabels;
use crate::loading::FontAssets;
use crate::GameState;
use bevy::core::Stopwatch;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameStopWatch>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_timer))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_timer.after(IceLabels::BreakIce)),
            );
    }
}

fn spawn_timer(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::Rgba {
                red: 0.7,
                green: 0.7,
                blue: 0.7,
                alpha: 0.7,
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "0.00".to_string(),
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(TimerText);
        });
}

#[derive(Component)]
struct TimerText;

struct GameStopWatch(Stopwatch);

impl Default for GameStopWatch {
    fn default() -> Self {
        GameStopWatch(Stopwatch::new())
    }
}

fn update_timer(
    time: Res<Time>,
    mut game_stop_watch: ResMut<GameStopWatch>,
    mut timer_text: Query<&mut Text, With<TimerText>>,
) {
    game_stop_watch.0.tick(time.delta());
    timer_text.single_mut().sections[0].value = format!("{:.2}", game_stop_watch.0.elapsed_secs());
}
