use bevy::prelude::*;

pub mod tile_type;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {}
}
