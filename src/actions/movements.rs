use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Reflect, Hash, Actionlike)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}