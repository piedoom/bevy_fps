mod systems;

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier3d::{
    physics::{NoUserData, RapierPhysicsPlugin},
    render::RapierRenderPlugin,
};
use blender_bevy_toolkit::BlendLoadPlugin;
pub(crate) use systems::{core::CorePlugin, movement::MovementPlugin};

use self::systems::assets::LoaderPlugin;

pub struct FullPlugins;

impl PluginGroup for FullPlugins {
    fn build(&mut self, app: &mut PluginGroupBuilder) {
        app.add(RapierPhysicsPlugin::<NoUserData>::default())
            .add(RapierRenderPlugin)
            .add(MovementPlugin)
            .add(CorePlugin)
            .add(LoaderPlugin)
            .add(BlendLoadPlugin::default());
    }
}
