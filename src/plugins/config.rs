use bevy::{prelude::*, reflect::TypeUuid};
use bevy_asset_ron::*;
use serde::Deserialize;

#[derive(Deserialize, TypeUuid, Debug, Default)]
#[uuid = "16170fe7-dcf0-e655-1422-d57a33356305"]
pub struct GameConfigAsset {
    pub damage: f32,
    pub durability: f32,
    pub min_level: u8,
}

#[derive(Default, Clone)]
pub struct ConfigHandles {
    pub handle: Handle<GameConfigAsset>,
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(
            // load `*.item` files
            RonAssetPlugin::<GameConfigAsset>::new(&["config"]),
        )
        .init_resource::<ConfigHandles>()
        .add_startup_system(setup.system());
    }
}

// TODO: 変更検知
fn setup(mut config_handles: ResMut<ConfigHandles>, server: Res<AssetServer>) {
    config_handles.handle = server.load("data.config");
    server.watch_for_changes().unwrap();
}

// usage
// fn print_config(assets: Res<Assets<GameConfigAsset>>, handles: Res<ConfigHandles>) {
//     if let Some(s) = assets.get(&handles.handle) {
//         println!("{:?}", s.damage)
//         // 変更を検知しないと毎回呼ばれる
//     }
// }
