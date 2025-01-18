use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;
use movements::Movement;
use serde::{Deserialize, Serialize};

pub mod movements;
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Reflect, Hash, Actionlike)]
pub enum Inputs {
    Movements(Movement),
    Delete,
    Spawn,
    None
}