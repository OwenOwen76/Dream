use crate::collision::system::*;
use crate::state::GameState;
use bevy::prelude::*;

pub mod map;
pub mod system;
pub mod tile_type;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionMapBuilt>().add_systems(
            Update,
            build_collision_map
                .run_if(resource_equals(CollisionMapBuilt(false)))
                .run_if(in_state(GameState::Playing)),
        );
    }
}
