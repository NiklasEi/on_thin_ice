use crate::loading::TextureAssets;
use crate::player::{AnimationTimer, Drowning};
use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub struct AnimalPlugin;

pub const ANIMAL_Z: f32 = 4.;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_animals))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_animals));
    }
}

#[derive(Component)]
pub struct Animal;

#[derive(Component)]
pub struct Walking(Vec2);

fn spawn_animals(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.animal.clone(),
            transform: Transform::from_translation(Vec3::new(150., 50., ANIMAL_Z)),
            ..Default::default()
        })
        .insert(Animal)
        .insert(Walking(Vec2::new(-1., -0.5)))
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}

fn move_animals(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Walking), (With<Animal>, Without<Drowning>)>,
) {
    let speed = 100.;
    for (mut transform, mut walking) in player_query.iter_mut() {
        let movement = Vec3::new(
            walking.0.x * speed * time.delta_seconds(),
            walking.0.y * speed * time.delta_seconds(),
            0.,
        );
        transform.translation += movement;
        transform.translation = transform.translation.clamp(
            Vec3::new(
                -WINDOW_WIDTH / 2. + 16.,
                -WINDOW_HEIGHT / 2. + 16.,
                ANIMAL_Z,
            ),
            Vec3::new(WINDOW_WIDTH / 2. - 16., WINDOW_HEIGHT / 2. - 16., ANIMAL_Z),
        );
        transform.rotation = Quat::from_rotation_z(-walking.0.angle_between(Vec2::new(0., 1.)));
    }
}
