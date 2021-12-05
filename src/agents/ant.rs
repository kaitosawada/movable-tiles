use bevy::prelude::*;
use super::Position;

pub struct Ant;

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
            // .add_system(position_transform.system());
    }
}

fn setup(mut commands: Commands) {
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         sprite: Sprite::new(Vec2::new(32.0, 32.0)),
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 0.0, 5.0),
    //             scale: Vec3::new(1.0, 1.0, 0.0),
    //             rotation: Quat::from_rotation_x(0.0),
    //         },
    //         ..Default::default()
    //     })
    //     .insert(Position { x: 0.0, y: 0.0, old_x: 0.0, old_y: 0.0 })
    //     .insert(Ant);
}

// fn position_transform(
//     timer: ResMut<GameTimer>,
//     mut query: Query<(&Position, &mut Transform), (With<Ant>,)>,
// ) {
//     query.iter_mut().for_each(|(pos, mut transform,)| {
//         transform.translation = Vec3::new(
//             ((pos.old_x as f64 + timer.0.percent() as f64 * (pos.x - pos.old_x) as f64)) as f32 * 32.0,
//             ((pos.old_y as f64 + timer.0.percent() as f64 * (pos.y - pos.old_y) as f64)) as f32 * 32.0,
//             5.0,
//         );
//     });
// }
