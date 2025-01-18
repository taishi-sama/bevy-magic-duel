use bevy::{app::App, DefaultPlugins};
use bevy_magic_duel::client::ClientPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ClientPlugin{})
            .run();
}
