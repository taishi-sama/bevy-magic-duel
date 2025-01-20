pub mod movement;

use avian2d::prelude::*;
use leafwing_input_manager::{prelude::{ActionState, InputMap}, InputManagerBundle};
use lightyear::prelude::*;
use bevy::prelude::*;

use crate::actions::{movements::Movement, Inputs};
#[derive(Debug, Clone, Component)]
#[require(Position, ActionState<Movement>, InputMap<Movement>, Collider, RigidBody, ColliderDensity, PrePredicted)]
pub struct Player;
#[derive(Debug, Clone, Component, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerId(pub ClientId); 
