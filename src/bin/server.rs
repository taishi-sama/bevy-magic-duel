use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::gizmos::GizmoPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::state::app::StatesPlugin;
use bevy::winit::WinitPlugin;
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
        .add_plugins(DefaultPlugins.set(
            RenderPlugin {
                render_creation: WgpuSettings {
                    backends: None,
                    ..default()
                }
                .into(),
                ..default()
            }).disable::<WinitPlugin>().disable::<GizmoPlugin>()
            ,)
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(ServerPlugin{})
            .run();
}
