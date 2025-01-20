use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}