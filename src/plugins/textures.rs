use bevy::{asset::LoadState, prelude::*};

#[derive(Default, Clone)]
pub struct SpriteHandles {
    pub handles: Vec<HandleUntyped>,
    pub texture_loaded: bool,
}

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .add_startup_system(setup.system())
            .add_system(load.system());
    }
}

fn setup(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    asset_server.watch_for_changes().unwrap();
}

fn load(
    mut sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.texture_loaded {
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        sprite_handles.texture_loaded = true;
    }
}
