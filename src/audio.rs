use crate::ice::IceLabels;
use crate::loading::AudioAssets;
use crate::player::{AnimalFallEvent, PlayerFallEvent};
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Channels>()
            .add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_walking))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(break_through_ice.after(IceLabels::CheckIceGrid)),
            )
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(start_audio));
    }
}

struct Channels {
    walking: AudioChannel,
}

impl Default for Channels {
    fn default() -> Self {
        Channels {
            walking: AudioChannel::new("walking".to_owned()),
        }
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_looped(audio_assets.background.clone());
}

fn start_walking(audio_assets: Res<AudioAssets>, audio: Res<Audio>, channels: Res<Channels>) {
    audio.set_volume_in_channel(0.3, &channels.walking);
    audio.play_looped_in_channel(audio_assets.walking.clone(), &channels.walking);
}

fn break_through_ice(
    mut player_fall_events: EventReader<PlayerFallEvent>,
    mut animal_fall_events: EventReader<AnimalFallEvent>,
    audio: Res<Audio>,
    channels: Res<Channels>,
    audio_assets: Res<AudioAssets>,
) {
    for _ in player_fall_events.iter() {
        audio.stop_channel(&channels.walking);
        audio.play(audio_assets.breaking_ice.clone());
    }
    for _ in animal_fall_events.iter() {
        audio.play(audio_assets.breaking_ice.clone());
    }
}
