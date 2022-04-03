use crate::loading::{CracksData, PixelData, TextureAssets};
use crate::player::{Player, PlayerFallEvent};
use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

const GRID_SIZE: usize = 10;
const GRID_X: usize = 80;
const GRID_Y: usize = 60;
const ICE_X: usize = 800;
const ICE_Y: usize = 600;
const CRACKS_X: usize = 32;
const CRACKS_Y: usize = 32;
const DATA_PER_PIXEL: usize = 4;
pub const ICE_HOLE_Z: f32 = 3.;

pub struct IcePlugin;

impl Plugin for IcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrackTheIceTimer>()
            .init_resource::<IceGrid>()
            .add_event::<BreakIceEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_ice))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(crack_the_ice)
                    .with_system(check_ice_grid.label(IceLabels::CheckIceGrid))
                    .with_system(
                        break_ice
                            .label(IceLabels::BreakIce)
                            .after(IceLabels::CheckIceGrid),
                    ),
            );
    }
}

fn spawn_ice(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(SpriteBundle {
        texture: textures.ice.clone(),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: textures.cracks_layer.clone(),
        ..Default::default()
    });
}

pub struct CrackTheIceTimer(Timer);

impl Default for CrackTheIceTimer {
    fn default() -> Self {
        CrackTheIceTimer(Timer::from_seconds(0.2, true))
    }
}

fn crack_the_ice(
    player: Query<&Transform, With<Player>>,
    mut images: ResMut<Assets<Image>>,
    textures: Res<TextureAssets>,
    cracks: Res<CracksData>,
    mut timer: ResMut<CrackTheIceTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    let player_transform = player.single();
    let cracks_layer = images
        .get_mut(textures.cracks_layer.clone())
        .expect("Failed to find the cracks_layer texture");
    let player_center = Vec2::new(
        player_transform.translation.x + ICE_X as f32 / 2.,
        ICE_Y as f32 / 2. - player_transform.translation.y,
    );
    let player_center: (usize, usize) = (
        player_center.x.clamp(0., ICE_X as f32 - 1.) as usize,
        player_center.y.clamp(0., ICE_Y as f32 - 1.) as usize,
    );

    let cracks = cracks.random();

    for PixelData {
        row,
        column,
        offset,
        data,
    } in cracks
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
        cracks_layer.data[ice_pixel + offset] =
            cracks_layer.data[ice_pixel + offset].saturating_add(*data);
    }
}

struct IceGrid {
    slots: Vec<Vec<SlotState>>,
}

impl Default for IceGrid {
    fn default() -> Self {
        let slots = Vec::from_iter(
            (0..GRID_Y).map(|_| Vec::from_iter((0..GRID_X).map(|_| SlotState::Ice))),
        );

        IceGrid { slots }
    }
}

#[derive(Copy, Clone)]
enum SlotState {
    Ice,
    Cracks { step: f64 },
    Brocken,
}

struct BreakIceEvent {
    position: Vec2,
}

fn check_ice_grid(
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut grid: ResMut<IceGrid>,
    mut break_ice_events: EventWriter<BreakIceEvent>,
    mut player_fall_event: EventWriter<PlayerFallEvent>,
) {
    let translation = player.single().translation;
    match update_and_return_ice_slot_state(
        time.seconds_since_startup(),
        translation.clone(),
        &mut grid,
    ) {
        WasUpdated::Yes(SlotState::Brocken) => {
            break_ice_events.send(BreakIceEvent {
                position: Vec2::new(translation.x, translation.y),
            });
            // only for player
            player_fall_event.send(PlayerFallEvent);
        }
        _ => (),
    }
}

enum WasUpdated<T> {
    Yes(T),
    No(T),
}

fn update_and_return_ice_slot_state(
    seconds_since_startup: f64,
    translation: Vec3,
    grid: &mut IceGrid,
) -> WasUpdated<SlotState> {
    let (x, y) = get_current_grid(translation);
    let state = grid.slots[y][x];
    if let SlotState::Ice = state {
        let cracked_state = SlotState::Cracks {
            step: seconds_since_startup,
        };
        grid.slots[y][x] = cracked_state;

        return WasUpdated::Yes(cracked_state);
    } else if let SlotState::Cracks { step } = state {
        if seconds_since_startup - step > 0.15 {
            grid.slots[y][x] = SlotState::Brocken;
            return WasUpdated::Yes(SlotState::Brocken);
        }
    }

    WasUpdated::No(state)
}

fn break_ice(
    mut events: EventReader<BreakIceEvent>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    for BreakIceEvent { position } in events.iter() {
        commands.spawn_bundle(SpriteBundle {
            texture: textures.hole.clone(),
            transform: Transform::from_xyz(position.x, position.y, ICE_HOLE_Z),
            ..Default::default()
        });
    }
}

fn get_current_grid(translation: Vec3) -> (usize, usize) {
    (
        ((translation.x + WINDOW_WIDTH / 2.) / GRID_SIZE as f32) as usize,
        ((translation.y + WINDOW_HEIGHT / 2.) / GRID_SIZE as f32) as usize,
    )
}

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum IceLabels {
    CheckIceGrid,
    BreakIce,
}
