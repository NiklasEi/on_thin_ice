use crate::ice::crack_border;
use crate::GameState;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;
use rand::random;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .init_resource::<CracksData>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

pub struct CracksLayer {
    pub layer: Handle<Image>,
}

impl FromWorld for CracksLayer {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut images = cell.get_resource_mut::<Assets<Image>>().unwrap();
        let cracks_data = cell.get_resource::<CracksData>().unwrap();
        let mut image = Image::new_fill(
            Extent3d {
                width: 800,
                height: 600,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[0u8, 0u8, 0u8, 0u8],
            TextureFormat::Rgba8UnormSrgb,
        );
        crack_border(&mut image, &cracks_data);
        let layer = images.add(image);

        CracksLayer { layer }
    }
}

pub struct CracksData {
    pub cracks_0: Vec<PixelData>,
    pub cracks_1: Vec<PixelData>,
}

impl CracksData {
    pub fn random(&self) -> &Vec<PixelData> {
        let weight: f32 = random();

        if weight > 0.5 {
            &self.cracks_0
        } else {
            &self.cracks_1
        }
    }
}

pub struct PixelData {
    pub row: usize,
    pub column: usize,
    pub offset: usize,
    pub data: u8,
}

impl FromWorld for CracksData {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let texture_assets = cell
            .get_resource::<TextureAssets>()
            .expect("Failed to get texture assets");
        let images = cell
            .get_resource::<Assets<Image>>()
            .expect("No image assets");
        let cracks_0_image = images
            .get(texture_assets.cracks_0.clone())
            .expect("No cracks image");
        let cracks_1_image = images
            .get(texture_assets.cracks_1.clone())
            .expect("No cracks image");

        CracksData {
            cracks_0: filter_image_data(cracks_0_image, 32, 32, 4),
            cracks_1: filter_image_data(cracks_1_image, 32, 32, 4),
        }
    }
}

fn filter_image_data(
    image: &Image,
    rows: usize,
    columns: usize,
    data_per_pixel: usize,
) -> Vec<PixelData> {
    let mut data = vec![];

    for row in 0..rows {
        for column in 0..columns {
            for offset in 0..data_per_pixel {
                let value = image.data[(row * columns + column) * data_per_pixel + offset];
                if value > 0 {
                    data.push(PixelData {
                        row,
                        column,
                        offset,
                        data: value,
                    })
                }
            }
        }
    }

    info!(
        "Reduced pixeldata from {} to {}",
        rows * columns * data_per_pixel,
        data.len()
    );

    data
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/walking.ogg")]
    pub walking: Handle<AudioSource>,
    #[asset(path = "audio/background.ogg")]
    pub background: Handle<AudioSource>,
    #[asset(path = "audio/ice_breaking.ogg")]
    pub breaking_ice: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 4, rows = 1))]
    #[asset(path = "textures/player.png")]
    pub player: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 300., tile_size_y = 300., columns = 3, rows = 1))]
    #[asset(path = "textures/countdown.png")]
    pub countdown: Handle<TextureAtlas>,
    #[asset(path = "textures/animal.png")]
    pub animal: Handle<Image>,
    #[asset(path = "textures/ice.png")]
    pub ice: Handle<Image>,
    #[asset(path = "textures/hole.png")]
    pub hole: Handle<Image>,
    #[asset(path = "textures/cracks_0.png")]
    pub cracks_0: Handle<Image>,
    #[asset(path = "textures/cracks_1.png")]
    pub cracks_1: Handle<Image>,
    #[asset(path = "textures/end.png")]
    pub end: Handle<Image>,
    #[asset(path = "textures/info.png")]
    pub info: Handle<Image>,
}
