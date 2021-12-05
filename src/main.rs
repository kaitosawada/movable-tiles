use bevy::prelude::*;
use movable_tiles::{
    agents::{ant::AntPlugin, player::PlayerPlugin},
    plugins::{chunk::*, config::*, textures::TexturePlugin, items::ItemsPlugin},
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigPlugin)
        .add_plugin(TexturePlugin)
        .add_plugin(ItemsPlugin)
        .add_plugin(ChunkPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AntPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
