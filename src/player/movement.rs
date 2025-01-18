use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::actions::{movements::Movement, Inputs};

const MAX_VELOCITY: f32 = 10.0;


pub(crate) fn shared_movement_behaviour(
    mut velocity: Mut<LinearVelocity>,
    action: &ActionState<Inputs>,
) {
    const MOVE_SPEED: f32 = 10.0;
    if action.pressed(&Inputs::Movements(Movement::Up)) {
        velocity.y += MOVE_SPEED;
    }
    if action.pressed(&Inputs::Movements(Movement::Down)) {
        velocity.y -= MOVE_SPEED;
    }
    if action.pressed(&Inputs::Movements(Movement::Left)) {
        velocity.x -= MOVE_SPEED;
    }
    if action.pressed(&Inputs::Movements(Movement::Right)) {
        velocity.x += MOVE_SPEED;
    }
    *velocity = LinearVelocity(velocity.clamp_length_max(MAX_VELOCITY));
}