use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_tilemap::{prelude::*, Tilemap};
use rand::{thread_rng, Rng};

use super::textures::SpriteHandles;

#[derive(Default, Clone)]
struct MapState {
    map_loaded: bool,
    atlas_loaded: bool,
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(TilemapDefaultPlugins)
            .init_resource::<MapState>()
            .add_system(load.system())
            .add_system(build_world.system());
    }
}

fn load(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    mut map_state: ResMut<MapState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    if map_state.atlas_loaded || !sprite_handles.texture_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    let tilemap = Tilemap::builder()
        .auto_chunk()
        .topology(GridTopology::Square)
        .dimensions(3, 3)
        .chunk_dimensions(64, 64, 1)
        .texture_dimensions(32, 32)
        .z_layers(3)
        .texture_atlas(atlas_handle)
        .finish()
        .unwrap();

    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Quat::from_rotation_x(0.0),
        },
        global_transform: Default::default(),
    };

    commands
        .spawn()
        .insert_bundle(tilemap_components)
        .insert(Timer::from_seconds(0.075, true));

    map_state.atlas_loaded = true;
}

fn build_world(
    mut map_state: ResMut<MapState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Tilemap,)>,
) {
    if map_state.map_loaded || !map_state.atlas_loaded {
        return;
    }
    for (mut map,) in query.iter_mut() {
        let chunk_width = (map.width().unwrap() * map.chunk_width()) as i32;
        let chunk_height = (map.height().unwrap() * map.chunk_height()) as i32;

        let floor1: Handle<Texture> = asset_server.get_handle("textures/square-floor.png");
        let floor2: Handle<Texture> = asset_server.get_handle("textures/square-floor_alt.png");
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let floor_index1 = texture_atlas.get_texture_index(&floor1).unwrap();
        let floor_index2 = texture_atlas.get_texture_index(&floor2).unwrap();
        println!("{}, {}", floor_index1, floor_index2);

        let mut rng = thread_rng();

        let mut tiles = Vec::new();
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let i: bool = rng.gen();
                let y = y - chunk_height / 2;
                let x = x - chunk_width / 2;
                let tile = Tile {
                    point: (x, y),
                    sprite_index: if x < 10 || y < 10 {floor_index1} else {floor_index2},
                    ..Default::default()
                };
                tiles.push(tile);
            }
        }
        map.insert_tiles(tiles).unwrap();

        map.spawn_chunk((-1, 0)).unwrap();
        map.spawn_chunk((0, 0)).unwrap();
        map.spawn_chunk((1, 0)).unwrap();
        map.spawn_chunk((-1, 1)).unwrap();
        map.spawn_chunk((0, 1)).unwrap();
        map.spawn_chunk((1, 1)).unwrap();
        map.spawn_chunk((-1, -1)).unwrap();
        map.spawn_chunk((0, -1)).unwrap();
        map.spawn_chunk((1, -1)).unwrap();

        map_state.map_loaded = true;
    }
}
