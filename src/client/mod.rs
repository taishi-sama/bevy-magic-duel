use avian2d::prelude::{LinearVelocity, Position};
use bevy::{color::palettes::css, math::vec2, prelude::*};
use client::{ClientCommands, ClientPlugins, PredictionSet, Replicate};
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::{ActionState, InputMap}};
use lightyear::prelude::*;
use crate::shared::SharedPlugin;
use crate::{actions::{movements::Movement, Inputs}, multiplayer::config::client_config, player::{movement::shared_movement_behaviour, Player, PlayerId}, protocol::{ProtocolPlugin, REPLICATION_GROUP}, states::ClientState};

pub struct ClientPlugin {
}
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>()
            .add_plugins(SharedPlugin)
            .add_plugins(ClientPlugins{config: client_config()})
            .add_plugins(ProtocolPlugin)
            .add_systems(Startup, init_client)
            .add_systems(
                PreUpdate,
                handle_connection
                    .after(MainSet::Receive)
                    .before(PredictionSet::SpawnPrediction),
            )
            .add_systems(FixedUpdate, player_movement);
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
                (Movement::Up, KeyCode::KeyW),
                (Movement::Down, KeyCode::KeyS),
                (Movement::Left, KeyCode::KeyA),
                (Movement::Right, KeyCode::KeyD),
            ]),
            Sprite::from_color(css::RED, vec2(10.0, 10.0)),
            PrePredicted::default(),
            Replicate{group: REPLICATION_GROUP, ..Default::default()}
        ));

    }
}

fn player_movement(
    tick_manager: Res<TickManager>,
    mut velocity_query: Query<
        (
            Entity,
            &PlayerId,
            &Position,
            &mut LinearVelocity,
            &ActionState<Movement>,
        ),
        With<client::Predicted>,
    >,
) {
    for (entity, player_id, position, velocity, action_state) in velocity_query.iter_mut() {
        if !action_state.get_pressed().is_empty() {
            trace!(?entity, tick = ?tick_manager.tick(), ?position, actions = ?action_state.get_pressed(), "applying movement to predicted player");
            // note that we also apply the input to the other predicted clients! even though
            //  their inputs are only replicated with a delay!
            // TODO: add input decay?
            shared_movement_behaviour(velocity, action_state);
        }
    }
}