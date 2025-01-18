use avian2d::prelude::Position;
use bevy::prelude::*;
use client::{ClientCommands, ClientPlugins, PredictionSet};
use leafwing_input_manager::prelude::InputMap;
use lightyear::prelude::*;

use crate::{actions::{movements::Movement, Inputs}, multiplayer::config::client_config, player::{Player, PlayerId}, states::ClientState};

pub struct ClientPlugin {
}
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>()
            .add_plugins(ClientPlugins{config: client_config()})
            .add_systems(Startup, init_client)
            .add_systems(
                PreUpdate,
                handle_connection
                    .after(MainSet::Receive)
                    .before(PredictionSet::SpawnPrediction),
            );;
    }
}


pub fn init_client(mut commands: Commands) {
    println!("Startup client!");
    commands.spawn(Camera2d);
    commands.connect_client();
}


pub(crate) fn handle_connection(
    mut commands: Commands,
    mut connection_event: EventReader<lightyear::client::events::ConnectEvent>,
) {
    for event in connection_event.read() {
        let client_id = event.client_id();
        commands.spawn((
            Text(format!("Client {}", client_id)),
            TextColor(Color::WHITE),
            TextFont::from_font_size(30.0),
        ));
        let y = (client_id.to_bits() as f32 * 50.0) % 500.0 - 250.0;
        // we will spawn two cubes per player, once is controlled with WASD, the other with arrows
        commands.spawn((
            PlayerId(client_id),
            Player,
            Position(Vec2::new(-50.0, y)),
            InputMap::new([
                (Inputs::Movements(Movement::Up), KeyCode::KeyW),
                (Inputs::Movements(Movement::Down), KeyCode::KeyS),
                (Inputs::Movements(Movement::Left), KeyCode::KeyA),
                (Inputs::Movements(Movement::Right), KeyCode::KeyD),
            ]),
        ));

    }
}