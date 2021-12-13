use std::collections::HashMap;

use bevy::{asset::LoadState, prelude::*, reflect::TypeUuid};
use bevy_asset_ron::RonAssetPlugin;
use serde::Deserialize;

use super::{textures::SpriteHandles, world::{AddItemToWorldEvent, Position}};

#[derive(Deserialize, TypeUuid, Debug, Default)]
#[uuid = "e0701840-8dc9-ff6b-80d1-b25acda6107f"]
pub struct ItemAssets {
    id: String,
    name: String,
    description: String,
    edible: bool,
    recovery_amount: u32,
    portable: bool,
    installable: bool,
    collision: bool,
    texture: String,
}

// Resource
#[derive(Default, Clone)]
struct ItemsHandles {
    handles: Vec<HandleUntyped>,
    ron_loaded: bool,
    material_loaded: bool,
}

#[derive(Default, Clone)]
pub struct ItemData {
    item_handle: Handle<ItemAssets>,
    material_handle: Handle<ColorMaterial>,
}

#[derive(Default, Clone)]
pub struct ItemDataMap {
    data: HashMap<String, ItemData>,
}

pub struct Item {
    // Component
    item_id: String,
}

pub struct Owner(pub Entity);

pub struct ItemsPlugin;

pub const ITEM_LAYER: f32 = 10.0;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(
            // load `*.item` files
            RonAssetPlugin::<ItemAssets>::new(&["item"]),
        )
        .init_resource::<ItemsHandles>()
        .init_resource::<ItemDataMap>()
        .add_startup_system(setup.system())
        .add_system(load_ron.system())
        .add_system(load_material.system())
        .add_system(fixup_textures.system());
    }
}

fn setup(mut prite_handles: ResMut<ItemsHandles>, server: Res<AssetServer>) {
    prite_handles.handles = server.load_folder("items").unwrap();
    server.watch_for_changes().unwrap();
}

fn load_ron(mut handles: ResMut<ItemsHandles>, asset_server: Res<AssetServer>) {
    if handles.ron_loaded {
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(handles.handles.iter().map(|handle| handle.id))
    {
        handles.ron_loaded = true;
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    event_writer: &mut EventWriter<AddItemToWorldEvent>,
    item_data: &ItemDataMap,
    item_id: &str,
    pos: Position,
) {
    if let Some(item_data) = item_data.data.get(item_id) {
        let entity = commands
            .spawn_bundle(SpriteBundle {
                material: item_data.material_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(1.0, 1.0, 0.0),
                    rotation: Quat::from_rotation_x(0.0),
                },
                ..Default::default()
            })
            .insert(Item {
                item_id: item_id.to_string(),
            })
            .insert(pos)
            .id();
        event_writer.send(AddItemToWorldEvent(entity, pos));
    }
}

fn load_material(
    mut commands: Commands,
    mut event_writer: EventWriter<AddItemToWorldEvent>,
    mut item_data: ResMut<ItemDataMap>,
    mut handles: ResMut<ItemsHandles>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<ItemAssets>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
) {
    if handles.material_loaded || !sprite_handles.textures_loaded {
        return;
    }

    let mut map = HashMap::new();
    for i in handles.handles.iter() {
        let item_assets = assets.get(i).unwrap();
        let texture: Handle<Texture> = asset_server.get_handle(item_assets.texture.as_str());
        let material_handle = material_assets.add(texture.into());
        let item = ItemData {
            item_handle: i.clone().typed(),
            material_handle,
        };
        map.insert(item_assets.id.clone(), item);
    }
    item_data.data = map;

    handles.material_loaded = true;

    // TODO: delete
    spawn_item(
        &mut commands,
        &mut event_writer,
        &(*item_data),
        "berry",
        Position { x: 2, y: 0 },
    );
    spawn_item(
        &mut commands,
        &mut event_writer,
        &(*item_data),
        "berry",
        Position { x: 3, y: 0 },
    );
    spawn_item(
        &mut commands,
        &mut event_writer,
        &(*item_data),
        "berry",
        Position { x: 0, y: 0 },
    );
    spawn_item(
        &mut commands,
        &mut event_writer,
        &(*item_data),
        "wall",
        Position { x: 1, y: 0 },
    );
}

fn fixup_textures(
    mut ev_asset: EventReader<AssetEvent<ItemAssets>>,
    assets: ResMut<Assets<ItemAssets>>,
    mut query: Query<(&mut Handle<ColorMaterial>, &Item)>,
    mut item_data: ResMut<ItemDataMap>,
    asset_server: Res<AssetServer>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { .. } => {}
            AssetEvent::Modified { handle } => {
                let item_assets = assets.get(handle).unwrap();
                let texture: Handle<Texture> =
                    asset_server.get_handle(item_assets.texture.as_str());
                let material_handle = material_assets.add(texture.into());
                let item = ItemData {
                    item_handle: handle.clone(),
                    material_handle: material_handle.clone(),
                };
                item_data.data.insert(item_assets.id.clone(), item);
                query.iter_mut().for_each(|(mut material, item)| {
                    if item.item_id == item_assets.id {
                        *material = material_handle.clone();
                    }
                })
            }
            AssetEvent::Removed { .. } => {}
        }
    }
}

// fn print_config(assets: Res<Assets<ItemAssets>>, handles: Res<ItemsHandles>) {
//     for handle in handles.handles.iter() {
//         let item_assets = assets.get(handle).unwrap();
//         println!("{:?}", item_assets.name);
//     }
// }
