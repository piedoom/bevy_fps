use crate::prelude::*;
use bevy::{asset::LoadState, prelude::*};

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Handle<DynamicScene>>()
            .init_resource::<Vec<HandleUntyped>>()
            .add_system_set(
                SystemSet::on_enter(GameState::Load).with_system(init_load_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Load).with_system(transition_system.system()),
            );
    }
}

pub type PreloadingAssets = Vec<HandleUntyped>;

fn init_load_system(
    mut state: ResMut<State<GameState>>,
    mut scene_handle: ResMut<Handle<DynamicScene>>,
    mut loading: ResMut<PreloadingAssets>,
    assets: Res<AssetServer>,
) {
    *scene_handle = assets.load("scenes/map.scn");
    loading.push(scene_handle.clone_untyped());
}

fn transition_system(
    mut state: ResMut<State<GameState>>,
    loading: Res<PreloadingAssets>,
    assets: Res<AssetServer>,
) {
    if loading
        .iter()
        .filter(|h| assets.get_load_state(*h) == LoadState::Loading)
        .count()
        == 0
    {
        // Transition states to the menu
        state.set(GameState::Main).ok();
    }
}
