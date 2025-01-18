pub mod movement;

use avian2d::prelude::*;
use leafwing_input_manager::{prelude::{ActionState, InputMap}, InputManagerBundle};
use lightyear::prelude::*;
use bevy::prelude::*;

use crate::actions::Inputs;
#[derive(Debug, Clone, Component)]
#[require(Position, ActionState<Inputs>, InputMap<Inputs>, Collider, RigidBody, ColliderDensity, PrePredicted)]
pub struct Player;
#[derive(Debug, Clone, Component)]
pub struct PlayerId(pub ClientId); 
