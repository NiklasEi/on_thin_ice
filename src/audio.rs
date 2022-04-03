use crate::ice::IceLabels;
use crate::loading::AudioAssets;
use crate::player::PlayerFallEvent;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Channels>()
            .add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(break_through_ice.after(IceLabels::CheckIceGrid)),
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
    audio.set_volume(0.3);
    audio.play_looped_in_channel(audio_assets.walking.clone(), &channels.walking);
}

fn break_through_ice(
    mut events: EventReader<PlayerFallEvent>,
    audio: Res<Audio>,
    channels: Res<Channels>,
) {
    for _ in events.iter() {
        audio.stop_channel(&channels.walking);
    }
}
