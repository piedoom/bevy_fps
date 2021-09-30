use bevy::{
    prelude::{App, Msaa},
    DefaultPlugins,
};
use bevy_rapier3d::physics::RapierPhysicsPlugin;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugins(fps::FullPlugins)
        .run();
}
