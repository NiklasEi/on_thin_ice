use crate::player::Drowning;
use crate::GameState;
use bevy::prelude::*;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(animate_walking)
                .with_system(animate_drowning)
                .with_system(animate_falling),
        );
    }
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Falling;

fn animate_walking(
    time: Res<Time>,
    mut query: Query<
        (&mut AnimationTimer, &mut TextureAtlasSprite),
        (Without<Drowning>, Without<Falling>),
    >,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 4;
        }
    }
}

fn animate_falling(
    mut command: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &mut AnimationTimer, &mut Transform),
        (Without<Drowning>, With<Falling>),
    >,
) {
    for (entity, mut timer, mut transform) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            transform.scale = Vec3::splat(1.);
            command.entity(entity).remove::<Falling>();
            continue;
        }
        let percentage = timer.0.percent();
        let scale = (1.0 - percentage) * 3. + 1.;
        transform.scale = Vec3::new(scale, scale, 1.);
    }
}

fn animate_drowning(time: Res<Time>, mut query: Query<(&mut Drowning, &mut Transform)>) {
    for (mut timer, mut transform) in query.iter_mut() {
        if timer.0.finished() {
            continue;
        }
        timer.0.tick(time.delta());
        transform.scale = Vec3::new(1.0 - timer.0.percent(), 1.0 - timer.0.percent(), 1.);
    }
}
