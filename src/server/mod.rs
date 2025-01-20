use avian2d::prelude::*;
use bevy::prelude::*;
use client::{Confirmed, Predicted};
use leafwing_input_manager::prelude::ActionState;
use lightyear::{prelude::*, shared::{events::components::MessageEvent, replication::components::InitialReplicated}};
use server::{ControlledBy, ServerCommands, ServerPlugins, ServerReplicationSet, SyncTarget};

use crate::{actions::{movements::Movement, Inputs}, multiplayer::config::server_config, player::{movement::shared_movement_behaviour, PlayerId}, protocol::{ProtocolPlugin, REPLICATION_GROUP}};
use crate::shared::SharedPlugin;
pub struct ServerPlugin {
    
}
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedPlugin)
            .add_plugins(ServerPlugins{config: server_config()})
            .add_plugins(ProtocolPlugin)
            .add_systems(Startup, init_server)
            .add_systems(PreUpdate, replicate_players.in_set(ServerReplicationSet::ClientReplication))
            .add_systems(FixedUpdate, movement)
            .add_systems(
                PreUpdate,
                // this system will replicate the inputs of a client to other clients
                // so that a client can predict other clients
                replicate_inputs.after(MainSet::EmitEvents),
            );
    }
}

pub fn init_server(mut commands: Commands) {
    commands.start_server();
}

pub(crate) fn replicate_players(
    mut commands: Commands,
    query: Query<(Entity, &InitialReplicated), (Added<InitialReplicated>, With<PlayerId>)>,
) {
    for (entity, replicated) in query.iter() {
        let client_id = replicated.client_id();
        info!("received player spawn event from client {client_id:?}");

        // for all player entities we have received, add a Replicate component so that we can start replicating it
        // to other clients
        if let Some(mut e) = commands.get_entity(entity) {
            // we want to replicate back to the original client, since they are using a pre-predicted entity
            let mut sync_target = SyncTarget::default();
            sync_target.interpolation = NetworkTarget::AllExceptSingle(client_id);

                // we want the other clients to apply interpolation for the player
            let replicate = server::Replicate {
                sync: sync_target,
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                    ..default()
                },
                // make sure that all entities that are predicted are part of the same replication group
                group: REPLICATION_GROUP,
                ..default()
            };
            e.insert((
                replicate,
                // if we receive a pre-predicted entity, only send the prepredicted component back
                // to the original client
                OverrideTargetComponent::<PrePredicted>::new(NetworkTarget::Single(client_id)),
                // not all physics components are replicated over the network, so add them on the server as well
                Collider::default(), RigidBody::default(), ColliderDensity::default()
            ));
        }
    }
}

pub(crate) fn movement(
    tick_manager: Res<TickManager>,
    mut action_query: Query<
        (
            Entity,
            &Position,
            &mut LinearVelocity,
            &ActionState<Movement>,
        ),
        (Without<Confirmed>, Without<Predicted>)

        // if we run in host-server mode, we don't want to apply this system to the local client's entities
        // because they are already moved by the client plugin
    >,
) {
    for (entity, position, velocity, action) in action_query.iter_mut() {
        if !action.get_pressed().is_empty() {
            // NOTE: be careful to directly pass Mut<PlayerPosition>
            // getting a mutable reference triggers change detection, unless you use `as_deref_mut()`
            shared_movement_behaviour(velocity, action);
            trace!(?entity, tick = ?tick_manager.tick(), ?position, actions = ?action.get_pressed(), "applying movement to player");
        }
    }
}


/// When we receive the input of a client, broadcast it to other clients
/// so that they can predict this client's movements accurately
pub(crate) fn replicate_inputs(
    mut connection: ResMut<lightyear::server::connection::ConnectionManager>,
    mut input_events: ResMut<Events<lightyear::server::events::MessageEvent<InputMessage<Movement>>>>,
) {
    for mut event in input_events.drain() {
        let client_id: ClientId = *event.context();

        // Optional: do some validation on the inputs to check that thesre's no cheating

        // rebroadcast the input to other clients
        connection
            .send_message_to_target::<InputChannel, _>(
                &mut event.message,
                NetworkTarget::AllExceptSingle(client_id),
            )
            .unwrap()
    }
}
