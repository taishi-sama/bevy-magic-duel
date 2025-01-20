use avian2d::{prelude::*, PhysicsPlugins};
use bevy::prelude::*;

pub struct SharedPlugin;
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    Main,
    // apply physics steps
    Physics,
}

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_plugins(
            PhysicsPlugins::new(FixedUpdate)
            .build()
            .disable::<ColliderHierarchyPlugin>()
        )
        .insert_resource(Gravity(Vec2::ZERO));

    }
}