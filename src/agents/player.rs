use bevy::{prelude::*, render::camera::Camera};

use super::{Position, SPEED};

#[derive(Default)]
struct GameTimer(Timer);

pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<GameTimer>()
            .insert_resource(GameTimer(Timer::new(
                // <--追加
                std::time::Duration::from_millis(50),
                true,
            )))
            .add_startup_system(setup.system())
            .add_system(game_timer.system().label("timer"))
            .add_system(input.system().label("input").after("timer"))
            .add_system(position_transform.system().after("input"));
    }
}

fn game_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    timer.0.tick(time.delta());
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
                rotation: Quat::from_rotation_x(0.0),
            },
            ..Default::default()
        })
        .insert(Position {
            x: 0,
            y: 0,
            old_x: 0,
            old_y: 0,
        })
        .insert(Player);
}

fn input(
    timer: ResMut<GameTimer>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Position,), (With<Player>,)>,
) {
    if !timer.0.finished() {
        return;
    }
    let mut old = Default::default();
    query.iter_mut().for_each(|(pos,)| {
        old = pos.clone();
    });
    if key_input.pressed(KeyCode::Left) {
        query.iter_mut().for_each(|(mut pos,)| {
            pos.x -= SPEED;
        });
    }
    if key_input.pressed(KeyCode::Right) {
        query.iter_mut().for_each(|(mut pos,)| {
            pos.x += SPEED;
        });
    }
    if key_input.pressed(KeyCode::Down) {
        query.iter_mut().for_each(|(mut pos,)| {
            pos.y -= SPEED;
        });
    }
    if key_input.pressed(KeyCode::Up) {
        query.iter_mut().for_each(|(mut pos,)| {
            pos.y += SPEED;
        });
    }
    query.iter_mut().for_each(|(mut pos,)| {
        pos.old_x = old.x;
        pos.old_y = old.y;
    });
}

fn position_transform(
    timer: ResMut<GameTimer>,
    mut query: QuerySet<(
        Query<(&Position, &mut Transform), (With<Player>,)>,
        Query<(&mut Transform,), (With<Camera>,)>,
    )>,
) {
    let mut vec = Default::default();
    let mut dx = 0;
    query.q0_mut().iter_mut().for_each(|(pos, mut transform)| {
        vec = Vec3::new(
            (pos.old_x as f64 + timer.0.percent() as f64 * (pos.x - pos.old_x) as f64) as f32
                * 32.0,
            (pos.old_y as f64 + timer.0.percent() as f64 * (pos.y - pos.old_y) as f64) as f32
                * 32.0,
            5.0,
        );
        dx = pos.x - pos.old_x;
        transform.translation = vec;
    });
    query.q1_mut().iter_mut().for_each(|(mut transform,)| {
        transform.translation = vec;
    });
}
