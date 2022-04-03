use crate::ice::IceLabels;
use crate::loading::TextureAssets;
use crate::player::{AnimalFallEvent, Drowning};
use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::math::Mat2;
use bevy::prelude::*;
use rand::random;

pub struct AnimalPlugin;

pub const ANIMAL_Z: f32 = 4.;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_animals))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_animals)
                    .with_system(drown_animals.after(IceLabels::CheckIceGrid)),
            );
    }
}

#[derive(Component)]
pub struct Animal;

#[derive(Component)]
pub struct Walking(Vec2);

#[derive(Component)]
pub struct Steering(Option<f32>);

fn spawn_animals(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.animal.clone(),
            transform: Transform::from_translation(Vec3::new(150., 50., ANIMAL_Z)),
            ..Default::default()
        })
        .insert(Animal)
        .insert(Walking(Vec2::new(-1., -0.5)))
        .insert(Steering(None));
}

fn move_animals(
    time: Res<Time>,
    mut player_query: Query<
        (&mut Transform, &mut Walking, &mut Steering),
        (With<Animal>, Without<Drowning>),
    >,
) {
    let speed = 50.;
    for (mut transform, mut walking, mut steering) in player_query.iter_mut() {
        let steering_rand: f32 = random();
        if let Some(steering_value) = steering.0.clone() {
            let rotation = Mat2::from_angle(-steering_value * time.delta_seconds());
            walking.0 = rotation.mul_vec2(walking.0);
            if steering_rand < 0.005 {
                steering.0 = None;
            }
        } else if steering_rand < 0.005 {
            steering.0 = Some(if steering_rand < 0.0025 {
                400. * steering_rand
            } else {
                -400. * steering_rand
            });
        }

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

fn drown_animals(mut commands: Commands, mut animal_fall_events: EventReader<AnimalFallEvent>) {
    for AnimalFallEvent(animal) in animal_fall_events.iter() {
        commands.entity(*animal).insert(Drowning);
    }
}
