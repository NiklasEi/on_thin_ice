use crate::ice::IceLabels;
use crate::loading::{FontAssets, TextureAssets};
use crate::menu::ButtonColors;
use crate::player::{Drowning, Player, PlayerFallEvent};
use crate::{GameState, Level};
use bevy::core::Stopwatch;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_high_score))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_timer))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(
                        update_timer
                            .label(UiLabels::UpdateTimer)
                            .after(IceLabels::BreakIce),
                    )
                    .with_system(update_high_score.after(UiLabels::UpdateTimer))
                    .with_system(player_fall.after(IceLabels::CheckIceGrid))
                    .with_system(spawn_restart_button.after(IceLabels::CheckIceGrid))
                    .with_system(click_restart_button.after(IceLabels::CheckIceGrid)),
            );
    }
}

#[derive(Default)]
struct HighScore(f32);

fn spawn_timer(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.insert_resource(GameStopWatch::default());
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
        .insert(Level)
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

fn spawn_high_score(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    right: Val::Px(10.),
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
                .insert(HighScoreText);
        });
}

#[derive(Component)]
struct TimerText;

#[derive(Component)]
struct HighScoreText;

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
    mut high_score: ResMut<HighScore>,
) {
    game_stop_watch.0.tick(time.delta());
    let score = game_stop_watch.0.elapsed_secs();
    timer_text.single_mut().sections[0].value = format!("{:.2}", score);
    if score > high_score.0 {
        high_score.0 = score;
    }
}

fn update_high_score(
    mut high_score_text: Query<&mut Text, With<HighScoreText>>,
    high_score: Res<HighScore>,
) {
    if !high_score.is_changed() {
        return;
    }
    high_score_text.single_mut().sections[0].value = format!("{:.2}", high_score.0);
}

struct RestartTimer(Timer);

impl Default for RestartTimer {
    fn default() -> Self {
        RestartTimer(Timer::from_seconds(3., false))
    }
}

fn player_fall(
    mut commands: Commands,
    mut events: EventReader<PlayerFallEvent>,
    mut game_stop_watch: ResMut<GameStopWatch>,
    textures: Res<TextureAssets>,
    player: Query<Entity, With<Player>>,
) {
    for _ in events.iter() {
        game_stop_watch.0.pause();
        commands.entity(player.single()).insert(Drowning::default());
        commands.insert_resource(RestartTimer::default());
        commands
            .spawn_bundle(SpriteBundle {
                texture: textures.end.clone(),
                ..SpriteBundle::default()
            })
            .insert(Level);
    }
}

fn spawn_restart_button(
    mut commands: Commands,
    time: Res<Time>,
    restart_timer: Option<ResMut<RestartTimer>>,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    if let Some(mut timer) = restart_timer {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.remove_resource::<RestartTimer>();
            commands
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(50.0)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button_colors.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Try again!".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    });
                });
        }
    }
}

fn click_restart_button(
    mut commands: Commands,
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                commands.entity(button).despawn_recursive();
                state.set(GameState::Restart).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum UiLabels {
    UpdateTimer,
}
