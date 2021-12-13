use bevy::{asset::LoadState, prelude::*};

#[derive(Default, Clone)]
pub struct SpriteHandles {
    pub textures_handles: Vec<HandleUntyped>,
    pub textures_loaded: bool,
    pub sprites_handles: Vec<HandleUntyped>,
    pub sprites_loaded: bool,
}

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .add_startup_system(setup.system())
            .add_system(textures_load.system())
            .add_system(sprites_load.system());
    }
}

fn setup(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.textures_handles = asset_server.load_folder("textures").unwrap();
    sprite_handles.sprites_handles = asset_server.load_folder("sprites").unwrap();
    asset_server.watch_for_changes().unwrap();
}

fn textures_load(
    mut sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.textures_loaded {
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.textures_handles.iter().map(|handle| handle.id))
    {
        sprite_handles.textures_loaded = true;
    }
}

fn sprites_load(
    mut sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.sprites_loaded {
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.sprites_handles.iter().map(|handle| handle.id))
    {
        sprite_handles.sprites_loaded = true;
    }
}
