use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::{prelude::*, shared::replication::components::InitialReplicated};
use server::{ControlledBy, ServerCommands, ServerPlugins, ServerReplicationSet, SyncTarget};

use crate::{multiplayer::config::server_config, player::PlayerId, protocol::REPLICATION_GROUP};

pub struct ServerPlugin {
    
}
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ServerPlugins{config: server_config()})
            .add_systems(Startup, init_server)
            .add_systems(PreUpdate, replicate_players.in_set(ServerReplicationSet::ClientReplication));
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


                // we want the other clients to apply interpolation for the player
            sync_target.interpolation = NetworkTarget::AllExceptSingle(client_id);
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
