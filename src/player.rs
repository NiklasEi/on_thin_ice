use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub const PLAYER_Z: f32 = 4.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
                .with_system(animate_player),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.player.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., PLAYER_Z)),
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerAnimationTimer(Timer::from_seconds(0.2, true)));
}

#[derive(Component)]
struct PlayerAnimationTimer(Timer);

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut PlayerAnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 4;
        }
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let speed = 100.;
    let movement = Vec3::new(
        actions.direction.x * speed * time.delta_seconds(),
        actions.direction.y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
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
            Quat::from_rotation_z(-actions.direction.angle_between(Vec2::new(0., 1.)));
    }
}
