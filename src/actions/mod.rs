use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;
use lightyear::prelude::LeafwingUserAction;
use movements::Movement;
use serde::{Deserialize, Serialize};

pub mod movements;
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum Inputs {
    Movements(Movement),
    Delete,
    Spawn,
    None
}