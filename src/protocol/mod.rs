use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::actions::Inputs;
use lightyear::client::components::{ComponentSyncMode, LerpFn};
use lightyear::client::interpolation::LinearInterpolator;
use lightyear::prelude::client;
use lightyear::prelude::server::{Replicate, SyncTarget};
use lightyear::prelude::*;
use lightyear::utils::avian2d::*;

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);
pub struct  ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin::<Inputs>::default());
        app.register_component::<Position>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(position::lerp)
        .add_correction_fn(position::lerp);

    app.register_component::<Rotation>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(rotation::lerp)
        .add_correction_fn(rotation::lerp);

    // NOTE: interpolation/correction is only needed for components that are visually displayed!
    // we still need prediction to be able to correctly predict the physics on the client
    app.register_component::<LinearVelocity>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<AngularVelocity>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full);

    }
}