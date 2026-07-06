mod camera;
mod characters;
mod collision;
mod debug;
mod map;
mod npc;
mod pathfinding;
mod state;

use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dreams".into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin,
            state::StatePlugin,
            map::MapPlugin,
            characters::CharactersPlugin,
            camera::CameraPlugin,
            debug::DebugPlugin,
            collision::CollisionPlugin,
        ))
        .run();
}
