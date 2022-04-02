use crate::loading::{CracksData, PixelData, TextureAssets};
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;

const ICE_X: usize = 800;
const ICE_Y: usize = 600;
const CRACKS_X: usize = 32;
const CRACKS_Y: usize = 32;
const DATA_PER_PIXEL: usize = 4;

pub struct IcePlugin;

impl Plugin for IcePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_ice))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(crack_the_ice));
    }
}

fn spawn_ice(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(SpriteBundle {
        texture: textures.ice.clone(),
        ..Default::default()
    });
}

fn crack_the_ice(
    player: Query<&Transform, With<Player>>,
    mut images: ResMut<Assets<Image>>,
    textures: Res<TextureAssets>,
    cracks: Res<CracksData>,
) {
    let player_transform = player.single();
    let ice_image = images
        .get_mut(textures.ice.clone())
        .expect("Failed to find the ice texture");
    let player_center = Vec2::new(
        player_transform.translation.x + ICE_X as f32 / 2.,
        ICE_Y as f32 / 2. - player_transform.translation.y,
    );
    let player_center: (usize, usize) = (
        player_center.x.clamp(0., ICE_X as f32 - 1.) as usize,
        player_center.y.clamp(0., ICE_Y as f32 - 1.) as usize,
    );

    for PixelData {
        row,
        column,
        offset,
        data,
    } in &cracks.cracks_0
    {
        let ice_index = (
            player_center.0 as i64 + *row as i64 - (CRACKS_X as f32 / 2.) as i64,
            player_center.1 as i64 + *column as i64 - (CRACKS_Y as f32 / 2.) as i64,
        );
        if ice_index.0 < 0
            || ice_index.0 >= ICE_X as i64
            || ice_index.1 < 0
            || ice_index.1 >= ICE_Y as i64
        {
            continue;
        };
        let ice_pixel = (ice_index.1 as usize * ICE_X + ice_index.0 as usize) * DATA_PER_PIXEL;
        ice_image.data[ice_pixel + offset] = *data;
    }
}
