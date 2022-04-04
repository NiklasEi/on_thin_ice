use crate::actions::Actions;
use crate::animal::Walking;
use crate::animate::AnimationTimer;
use crate::ice::{get_random_direction, get_random_spawn_point, IceLabels, SpawnPoints};
use crate::loading::TextureAssets;
use crate::{GameState, Level, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::math::Mat2;
use bevy::prelude::*;

pub const PLAYER_Z: f32 = 5.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerFallEvent>()
            .add_event::<AnimalFallEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Countdown).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player.before(IceLabels::CheckIceGrid)),
            );
    }
}

pub struct PlayerFallEvent;
pub struct AnimalFallEvent(pub Entity);

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut spawn_points: ResMut<SpawnPoints>,
) {
    let random_spawn_point = get_random_spawn_point(&mut spawn_points);
    let random_direction = get_random_direction();
    let mut transform = Transform::from_translation(Vec3::new(
        random_spawn_point.x,
        random_spawn_point.y,
        PLAYER_Z,
    ));
    transform.rotation = Quat::from_rotation_z(-random_direction.angle_between(Vec2::new(0., 1.)));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.player.clone(),
            transform,
            ..Default::default()
        })
        .insert(Player)
        .insert(Level)
        .insert(Walking(random_direction))
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}

#[derive(Component)]
pub struct Drowning(pub Timer);

impl Default for Drowning {
    fn default() -> Self {
        Drowning(Timer::from_seconds(3., false))
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut Walking), (With<Player>, Without<Drowning>)>,
) {
    let speed = 70.;
    for (mut player_transform, mut walking) in player_query.iter_mut() {
        if let Some(steering) = actions.steering {
            let rotation = Mat2::from_angle(-steering * time.delta_seconds());
            walking.0 = rotation.mul_vec2(walking.0);
        }
        let movement = Vec3::new(
            walking.0.x * speed * time.delta_seconds(),
            walking.0.y * speed * time.delta_seconds(),
            0.,
        );
        player_transform.translation += movement;
        player_transform.translation = player_transform.translation.clamp(
            Vec3::new(
                -WINDOW_WIDTH / 2. + 16.,
                -WINDOW_HEIGHT / 2. + 16.,
                PLAYER_Z,
            ),
            Vec3::new(WINDOW_WIDTH / 2. - 16., WINDOW_HEIGHT / 2. - 16., PLAYER_Z),
        );
        player_transform.rotation =
            Quat::from_rotation_z(-walking.0.angle_between(Vec2::new(0., 1.)));
    }
}
