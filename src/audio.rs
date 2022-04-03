use crate::ice::IceLabels;
use crate::loading::AudioAssets;
use crate::player::{AnimalFallEvent, PlayerFallEvent};
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};
use rand::random;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Channels>()
            .add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(break_through_ice.after(IceLabels::CheckIceGrid))
                    .with_system(random_ice_cracking),
            );
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

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, channels: Res<Channels>) {
    audio.set_volume_in_channel(0.3, &channels.walking);
    audio.play_looped_in_channel(audio_assets.walking.clone(), &channels.walking);
    audio.play_looped(audio_assets.background.clone());
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

fn random_ice_cracking(audio: Res<Audio>, audio_assets: Res<AudioAssets>) {
    let rand: f32 = random();
    if rand < 0.002 {
        let percent_rand = rand * 500.;
        let audio_handle = if percent_rand < 0.25 {
            audio_assets.ice_background_0.clone()
        } else if percent_rand < 0.5 {
            audio_assets.ice_background_1.clone()
        } else if percent_rand < 0.75 {
            audio_assets.ice_background_2.clone()
        } else {
            audio_assets.ice_background_3.clone()
        };
        audio.play(audio_handle);
    }
}
