use crate::GameState;
use bevy::math::Mat2;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

pub struct Actions {
    pub direction: Vec2,
    pub steering: Option<f32>,
}

impl Default for Actions {
    fn default() -> Self {
        Actions {
            direction: Vec2::new(0., 1.),
            steering: None,
        }
    }
}

fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
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

    if let Some(steering) = actions.steering {
        let rotation = Mat2::from_angle(-steering * time.delta_seconds());
        actions.direction = rotation.mul_vec2(actions.direction);
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

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
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
