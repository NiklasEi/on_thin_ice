mod actions;
mod animal;
mod audio;
mod ice;
mod loading;
mod menu;
mod player;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use crate::animal::AnimalPlugin;
use crate::ice::IcePlugin;
use crate::ui::UiPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 800.;
pub const WINDOW_HEIGHT: f32 = 600.;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
    Restart,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(IcePlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AnimalPlugin);

        app.add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup_cameras))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(despawn_level))
            .add_system_set(SystemSet::on_enter(GameState::Restart).with_system(restart));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

#[derive(Component)]
pub struct Level;

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn restart(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Playing).unwrap();
}

fn despawn_level(mut commands: Commands, level_entities: Query<Entity, With<Level>>) {
    for entity in level_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
