use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::actions::{movements::Movement, Inputs};

const MAX_VELOCITY: f32 = 100.0;


pub(crate) fn shared_movement_behaviour(
    mut velocity: Mut<LinearVelocity>,
    action: &ActionState<Movement>,
) {
    const MOVE_SPEED: f32 = 100.0;
    if action.pressed(&Movement::Up) {
        debug!("W pressed");
        velocity.y += MOVE_SPEED;
    }
    if action.pressed(&Movement::Down) {
        debug!("S pressed");
        velocity.y -= MOVE_SPEED;
    }
    if action.pressed(&Movement::Left) {
        debug!("A pressed");
        velocity.x -= MOVE_SPEED;
    }
    if action.pressed(&Movement::Right) {
        debug!("D pressed");
        velocity.x += MOVE_SPEED;
    }
    *velocity = LinearVelocity(velocity.clamp_length_max(MAX_VELOCITY));
}