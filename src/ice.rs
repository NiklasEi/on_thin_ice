use crate::animal::Animal;
use crate::animate::Falling;
use crate::loading::{CracksData, CracksLayer, PixelData, TextureAssets};
use crate::player::{AnimalFallEvent, Drowning, Player, PlayerFallEvent};
use crate::{GameState, Level, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::math::Mat2;
use bevy::prelude::*;
use rand::random;
use std::f32::consts::PI;

const GRID_SIZE: usize = 10;
const GRID_X: usize = 80;
const GRID_Y: usize = 60;
const ICE_X: usize = 800;
const ICE_Y: usize = 600;
const CRACKS_X: usize = 32;
const CRACKS_Y: usize = 32;
const DATA_PER_PIXEL: usize = 4;
pub const ICE_HOLE_Z: f32 = 3.;
pub const SPAWN_BORDER: f32 = 200.;

pub struct IcePlugin;

impl Plugin for IcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrackTheIceTimer>()
            .add_event::<BreakIceEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::Countdown)
                    .with_system(prepare_cracks_layer.exclusive_system().at_start()),
            )
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_ice))
            .add_system_set(
                SystemSet::on_enter(GameState::Countdown).with_system(spawn_cracks_layer),
            )
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

fn prepare_cracks_layer(world: &mut World) {
    let cracks_layer = CracksLayer::from_world(world);
    world.insert_resource(cracks_layer);
    world.insert_resource(SpawnPoints(vec![]));
}

pub struct SpawnPoints(pub Vec<Vec2>);

fn spawn_ice(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(SpriteBundle {
        texture: textures.ice.clone(),
        ..Default::default()
    });
}

fn spawn_cracks_layer(mut commands: Commands, textures: Res<CracksLayer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.layer.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        })
        .insert(Level);
    commands.insert_resource(IceGrid::default());
}

pub struct CrackTheIceTimer(Timer);

impl Default for CrackTheIceTimer {
    fn default() -> Self {
        CrackTheIceTimer(Timer::from_seconds(0.3, true))
    }
}

fn crack_the_ice(
    player: Query<
        &Transform,
        (
            With<Player>,
            Without<Animal>,
            Without<Falling>,
            Without<Drowning>,
        ),
    >,
    animals: Query<
        &Transform,
        (
            With<Animal>,
            Without<Player>,
            Without<Falling>,
            Without<Drowning>,
        ),
    >,
    mut images: ResMut<Assets<Image>>,
    textures: Res<CracksLayer>,
    cracks: Res<CracksData>,
    mut timer: ResMut<CrackTheIceTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    let cracks_layer = images
        .get_mut(textures.layer.clone())
        .expect("Failed to find the cracks_layer texture");
    if let Ok(player_transform) = player.get_single() {
        crack_ice_at(&player_transform.translation, &cracks, cracks_layer);
    }

    for animal_transform in animals.iter() {
        crack_ice_at(&animal_transform.translation, &cracks, cracks_layer);
    }
}

fn crack_ice_at(translation: &Vec3, cracks: &CracksData, cracks_layer: &mut Image) {
    let center = Vec2::new(
        translation.x + ICE_X as f32 / 2.,
        ICE_Y as f32 / 2. - translation.y,
    );
    let center: (usize, usize) = (
        center.x.clamp(0., ICE_X as f32 - 1.) as usize,
        center.y.clamp(0., ICE_Y as f32 - 1.) as usize,
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
            center.0 as i64 + *row as i64 - (CRACKS_X as f32 / 2.) as i64,
            center.1 as i64 + *column as i64 - (CRACKS_Y as f32 / 2.) as i64,
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

pub fn crack_border(image: &mut Image, cracks_data: &CracksData) {
    let distance = 16.;
    let per_row = (WINDOW_WIDTH / distance) as usize;
    let per_column = (WINDOW_HEIGHT / distance) as usize;
    for index in 1..per_row {
        crack_ice_at(
            &Vec3::new(
                index as f32 * distance - WINDOW_WIDTH / 2.,
                (WINDOW_HEIGHT / 2.) - distance,
                0.,
            ),
            cracks_data,
            image,
        );
        crack_ice_at(
            &Vec3::new(
                index as f32 * distance - WINDOW_WIDTH / 2.,
                -(WINDOW_HEIGHT / 2.) + distance,
                0.,
            ),
            cracks_data,
            image,
        );
    }

    for index in 2..per_column - 1 {
        crack_ice_at(
            &Vec3::new(
                -WINDOW_WIDTH / 2. + distance,
                index as f32 * distance - (WINDOW_HEIGHT / 2.),
                0.,
            ),
            cracks_data,
            image,
        );
        crack_ice_at(
            &Vec3::new(
                WINDOW_WIDTH / 2. - distance,
                index as f32 * distance - (WINDOW_HEIGHT / 2.),
                0.,
            ),
            cracks_data,
            image,
        );
    }
}

struct IceGrid {
    slots: Vec<Vec<SlotState>>,
}

impl Default for IceGrid {
    fn default() -> Self {
        let mut slots = Vec::from_iter(
            (0..GRID_Y).map(|_| Vec::from_iter((0..GRID_X).map(|_| SlotState::Ice))),
        );
        slots[0] = Vec::from_iter((0..GRID_X).map(|_| SlotState::Cracks { step: 0. }));
        slots[1] = Vec::from_iter((0..GRID_X).map(|_| SlotState::Cracks { step: 0. }));
        slots[GRID_Y - 1] = Vec::from_iter((0..GRID_X).map(|_| SlotState::Cracks { step: 0. }));
        slots[GRID_Y - 2] = Vec::from_iter((0..GRID_X).map(|_| SlotState::Cracks { step: 0. }));

        for row in 0..GRID_Y {
            slots[row][0] = SlotState::Cracks { step: 0. };
            slots[row][1] = SlotState::Cracks { step: 0. };
            slots[row][GRID_X - 1] = SlotState::Cracks { step: 0. };
            slots[row][GRID_X - 2] = SlotState::Cracks { step: 0. };
        }

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
    player: Query<&Transform, (With<Player>, Without<Animal>, Without<Drowning>)>,
    animals: Query<
        (Entity, &Transform),
        (
            With<Animal>,
            Without<Player>,
            Without<Drowning>,
            Without<Falling>,
        ),
    >,
    time: Res<Time>,
    mut grid: ResMut<IceGrid>,
    mut break_ice_events: EventWriter<BreakIceEvent>,
    mut player_fall_event: EventWriter<PlayerFallEvent>,
    mut animal_fall_event: EventWriter<AnimalFallEvent>,
) {
    for (entity, Transform { translation, .. }) in animals.iter() {
        match update_and_return_ice_slot_state(
            time.seconds_since_startup(),
            &translation,
            &mut grid,
        ) {
            WasUpdated::Yes(SlotState::Brocken) => {
                break_ice_events.send(BreakIceEvent {
                    position: Vec2::new(translation.x, translation.y),
                });
                animal_fall_event.send(AnimalFallEvent(entity));
            }
            _ => (),
        }
    }
    for Transform { translation, .. } in player.iter() {
        match update_and_return_ice_slot_state(
            time.seconds_since_startup(),
            &translation,
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
}

enum WasUpdated<T> {
    Yes(T),
    No(T),
}

fn update_and_return_ice_slot_state(
    seconds_since_startup: f64,
    translation: &Vec3,
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
        if seconds_since_startup - step > 0.4 {
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
        commands
            .spawn_bundle(SpriteBundle {
                texture: textures.hole.clone(),
                transform: Transform::from_xyz(position.x, position.y, ICE_HOLE_Z),
                ..Default::default()
            })
            .insert(Level);
    }
}

fn get_current_grid(translation: &Vec3) -> (usize, usize) {
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

pub fn get_random_spawn_point(spawn_points: &mut SpawnPoints) -> Vec2 {
    'attempt: loop {
        let point = get_random_point(SPAWN_BORDER);
        for spawn_point in &spawn_points.0 {
            if spawn_point.distance(point) < 100. {
                continue 'attempt;
            }
        }
        spawn_points.0.push(point.clone());
        return point;
    }
}

pub fn get_random_point(border: f32) -> Vec2 {
    let rand_x: f32 = random();
    let rand_y: f32 = random();

    let range_x = WINDOW_WIDTH - 2. * border;
    let range_y = WINDOW_HEIGHT - 2. * border;

    Vec2::new(
        range_x * rand_x + border - WINDOW_WIDTH / 2.,
        range_y * rand_y + border - WINDOW_HEIGHT / 2.,
    )
}

pub fn get_random_direction() -> Vec2 {
    let rand: f32 = random();
    let rotation = Mat2::from_angle(rand * 2. * PI);
    rotation.mul_vec2(Vec2::new(1., 0.))
}
