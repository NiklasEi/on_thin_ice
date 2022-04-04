use crate::animate::{AnimationTimer, Falling};
use crate::ice::{
    get_random_direction, get_random_point, get_random_spawn_point, IceLabels, SpawnPoints,
};
use crate::loading::TextureAssets;
use crate::player::{AnimalFallEvent, Drowning};
use crate::{GameState, Level, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::math::Mat2;
use bevy::prelude::*;
use rand::random;

pub struct AnimalPlugin;

pub const ANIMAL_Z: f32 = 4.;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Countdown).with_system(spawn_initial_animals),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_animals.before(IceLabels::CheckIceGrid))
                .with_system(drown_animals.after(IceLabels::CheckIceGrid))
                .with_system(spawn_animals),
        );
    }
}

#[derive(Component)]
struct SpawnAnimalTimer(Timer);

fn spawn_animals(
    mut commands: Commands,
    mut timer: ResMut<SpawnAnimalTimer>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    let random_spawn_point = get_random_point(100.);
    let random_direction = get_random_direction();
    let mut transform = Transform::from_translation(Vec3::new(
        random_spawn_point.x,
        random_spawn_point.y,
        ANIMAL_Z,
    ));
    transform.rotation = Quat::from_rotation_z(-random_direction.angle_between(Vec2::new(0., 1.)));
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.animal.clone(),
            transform,
            ..Default::default()
        })
        .insert(Level)
        .insert(Animal)
        .insert(Falling)
        .insert(AnimationTimer(Timer::from_seconds(2., false)))
        .insert(Walking(random_direction))
        .insert(Steering(None));
}

#[derive(Component)]
pub struct Animal;

#[derive(Component)]
pub struct Walking(pub Vec2);

#[derive(Component)]
pub struct Steering(Option<f32>);

fn spawn_initial_animals(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut spawn_points: ResMut<SpawnPoints>,
) {
    commands.insert_resource(SpawnAnimalTimer(Timer::from_seconds(10., true)));
    for _ in 0..5 {
        let random_spawn_point = get_random_spawn_point(&mut spawn_points);
        let random_direction = get_random_direction();
        let mut transform = Transform::from_translation(Vec3::new(
            random_spawn_point.x,
            random_spawn_point.y,
            ANIMAL_Z,
        ));
        transform.rotation =
            Quat::from_rotation_z(-random_direction.angle_between(Vec2::new(0., 1.)));
        commands
            .spawn_bundle(SpriteBundle {
                texture: textures.animal.clone(),
                transform,
                ..Default::default()
            })
            .insert(Level)
            .insert(Animal)
            .insert(Walking(random_direction))
            .insert(Steering(None));
    }
}

fn move_animals(
    time: Res<Time>,
    mut player_query: Query<
        (&mut Transform, &mut Walking, &mut Steering),
        (With<Animal>, Without<Drowning>, Without<Falling>),
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
        commands.entity(*animal).insert(Drowning::default());
    }
}
