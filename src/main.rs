use bevy::prelude::*;
use movable_tiles::{
    agents::{ant::AntPlugin, player::PlayerPlugin},
    plugins::{chunk::*, config::*, items::ItemsPlugin, textures::TexturePlugin, world::WorldPlugin},
};


fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 1024.,
            height: 576.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(TexturePlugin)
        .add_plugin(ItemsPlugin)
        .add_plugin(ChunkPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AntPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn_bundle(camera_bundle);
}
