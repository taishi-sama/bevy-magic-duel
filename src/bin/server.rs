use bevy::diagnostic::DiagnosticsPlugin;
use bevy::state::app::StatesPlugin;
use bevy::{log::LogPlugin, utils::default, MinimalPlugins};
use bevy::prelude::*;
use bevy_magic_duel::server::ServerPlugin;
use tracing::Level;


fn log_plugin() -> LogPlugin {
    LogPlugin {
        level: Level::INFO,
        filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,bevy_time=warn".to_string(),
        ..default()
    }
}
fn main() {
    App::new()
        .add_plugins((        MinimalPlugins,
            log_plugin(),
            StatesPlugin,
            HierarchyPlugin,
            DiagnosticsPlugin,))
        .add_plugins(ServerPlugin{})
            .run();
}
