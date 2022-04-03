use crate::actions::Actions;
use crate::animate::AnimationTimer;
use crate::loading::TextureAssets;
use crate::{GameState, Level, WINDOW_HEIGHT, WINDOW_WIDTH};
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
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player));
    }
}

pub struct PlayerFallEvent;
pub struct AnimalFallEvent(pub Entity);

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.player.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., PLAYER_Z)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Level)
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
    mut player_query: Query<&mut Transform, (With<Player>, Without<Drowning>)>,
) {
    let speed = 70.;
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
