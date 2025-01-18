use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientState {
    Menu,
    Game,
}
impl Default for ClientState {
    fn default() -> Self {
        Self::Menu
    }
}