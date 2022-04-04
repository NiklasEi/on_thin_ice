use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(initialize.exclusive_system()),
        );
    }
}

fn initialize(world: &mut World) {
    let actions = Actions::from_world(world);
    world.insert_resource(actions);
}

pub struct Actions {
    pub steering: Option<f32>,
}

impl FromWorld for Actions {
    fn from_world(world: &mut World) -> Self {
        let input = world.get_resource::<Input<KeyCode>>().unwrap();
        if GameControl::Left.pressed(input) && GameControl::Right.pressed(input) {
            Actions { steering: None }
        } else if GameControl::Left.pressed(input) {
            Actions {
                steering: Some(-1.2),
            }
        } else if GameControl::Right.pressed(input) {
            Actions {
                steering: Some(1.2),
            }
        } else {
            Actions { steering: None }
        }
    }
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Left.just_released(&keyboard_input)
        || GameControl::Left.pressed(&keyboard_input)
        || GameControl::Right.just_released(&keyboard_input)
        || GameControl::Right.pressed(&keyboard_input)
    {
        let steering;

        if GameControl::Right.just_released(&keyboard_input)
            || GameControl::Left.just_released(&keyboard_input)
        {
            if GameControl::Right.pressed(&keyboard_input) {
                steering = 1.2;
            } else if GameControl::Left.pressed(&keyboard_input) {
                steering = -1.2;
            } else {
                steering = 0.;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input) {
            steering = 1.2;
        } else if GameControl::Left.just_pressed(&keyboard_input) {
            steering = -1.2;
        } else {
            steering = actions.steering.unwrap_or(0.);
        }
        actions.steering = Some(steering);
    } else {
        actions.steering = None;
    }
}

enum GameControl {
    Left,
    Right,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
        }
    }

    fn pressed(&self, keyboard_input: &Input<KeyCode>) -> bool {
        match self {
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
        }
    }
}
