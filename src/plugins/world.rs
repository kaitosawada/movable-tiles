use bevy::{prelude::*, render::camera::Camera};
use std::collections::HashMap;

use crate::agents::player::PlayerState;

use super::items::{Item, Owner};

pub const YZ_PROJECTION_RATIO: f32 = -1.0;
pub const Z_OFFSET: f32 = 500.0;

#[derive(Clone, Default, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Default, PartialEq, Copy)]
pub struct TextureOffset {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Default)]
pub struct OldPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct GameWorld {
    // Component
    pub item_map: HashMap<Position, Entity>,
}

pub struct AddItemToWorldEvent(pub Entity, pub Position);
pub struct RemoveItemFromWorldEvent(pub Entity, pub Position);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddItemToWorldEvent>()
            .add_event::<RemoveItemFromWorldEvent>()
            .add_startup_system(setup.system())
            .add_system(add_item.system())
            .add_system(remove_item.system())
            .add_system(world_position_transform.system())
            .add_system(world_position_transform_delta.system().label("position"))
            .add_system(camera_transform.system().after("position"))
            .add_system(item_position_transform.system().after("position"));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(GameWorld::default());
}

fn add_item(
    mut event_reader: EventReader<AddItemToWorldEvent>,
    mut world_query: Query<(&mut GameWorld,)>,
) {
    for ev in event_reader.iter() {
        for (mut world,) in world_query.iter_mut() {
            world.item_map.insert(ev.1, ev.0);
        }
    }
}

fn remove_item(
    mut event_reader: EventReader<RemoveItemFromWorldEvent>,
    mut world_query: Query<(&mut GameWorld,)>,
) {
    for ev in event_reader.iter() {
        for (mut world,) in world_query.iter_mut() {
            world.item_map.remove(&ev.1);
        }
    }
}

fn world_position_transform(
    mut query: Query<(&Position, &mut Transform), (Without<OldPosition>,)>,
) {
    query.iter_mut().for_each(|(pos, mut transform)| {
        transform.translation = Vec3::new(
            pos.x as f32 * 32.0,
            pos.y as f32 * 32.0,
            pos.y as f32 * YZ_PROJECTION_RATIO + Z_OFFSET,
        );
    });
}

fn world_position_transform_delta(
    mut query: Query<(
        &Position,
        &OldPosition,
        &mut Transform,
        &Timer,
        &TextureOffset,
    )>,
) {
    query
        .iter_mut()
        .for_each(|(pos, old_pos, mut transform, timer, offset)| {
            if f32::is_nan(timer.percent()) {
                transform.translation = Vec3::new(
                    pos.x as f32 * 32.0 + offset.x,
                    pos.y as f32 * 32.0 + offset.y,
                    pos.y as f32 * YZ_PROJECTION_RATIO + Z_OFFSET,
                );
            } else {
                transform.translation = Vec3::new(
                    (old_pos.x as f64 + timer.percent() as f64 * (pos.x - old_pos.x) as f64) as f32
                        * 32.0
                        + offset.x,
                    (old_pos.y as f64 + timer.percent() as f64 * (pos.y - old_pos.y) as f64) as f32
                        * 32.0
                        + offset.y,
                    (old_pos.y as f64 + timer.percent() as f64 * (pos.y - old_pos.y) as f64) as f32
                        * YZ_PROJECTION_RATIO
                        + Z_OFFSET,
                );
            }
        });
}

fn camera_transform(
    mut query: QuerySet<(
        Query<(&Transform,), (With<PlayerState>,)>,
        Query<(&mut Transform,), (With<Camera>,)>,
    )>,
) {
    let mut vec = Default::default();
    query.q0_mut().iter_mut().for_each(|(transform,)| {
        vec = transform.translation;
    });
    query.q1_mut().iter_mut().for_each(|(mut transform,)| {
        transform.translation = Vec3::new(vec.x, vec.y, 1000.0 - 0.1);
    });
}

const OFFSET_X: f32 = -12.0;
const OFFSET_Y: f32 = -6.0;

const OFFSET_X_DROP: f32 = 0.0;
const OFFSET_Y_DROP: f32 = -28.0;

fn item_position_transform(
    mut query: QuerySet<(
        Query<(Entity, &mut Transform), (With<Item>,)>,
        Query<(Entity, &Owner), (With<Item>,)>,
        Query<(&PlayerState, &Transform, &Timer)>,
    )>,
) {
    let mut map: HashMap<Entity, (f32, f32, f32)> = HashMap::new();
    query.q1().iter().for_each(|(item_entity, owner)| {
        if let Result::Ok((state, parent_transform, timer)) = query.q2().get(owner.0) {
            let pick_move = || {
                (
                    parent_transform.translation.x
                        + OFFSET_X_DROP
                        + f32::max(timer.percent() * 1.8 - 0.8, 0.0) * (OFFSET_X - OFFSET_X_DROP),
                    parent_transform.translation.y
                        + OFFSET_Y_DROP
                        + f32::max(timer.percent() * 1.8 - 0.8, 0.0) * (OFFSET_Y - OFFSET_Y_DROP),
                    parent_transform.translation.z,
                )
            };
            let drop_move = || {
                (
                    parent_transform.translation.x
                        + OFFSET_X
                        + f32::min(timer.percent() * 1.5, 1.0) * (OFFSET_X_DROP - OFFSET_X),
                    parent_transform.translation.y
                        + OFFSET_Y
                        + f32::min(timer.percent() * 1.5, 1.0) * (OFFSET_Y_DROP - OFFSET_Y),
                    parent_transform.translation.z,
                )
            };
            map.insert(
                item_entity,
                match *state {
                    PlayerState::Pick(_) => pick_move(),
                    PlayerState::Drop => drop_move(),
                    PlayerState::PickAndDrop(item_entity_event) => {
                        // guardの方が見やすい？
                        if item_entity_event != item_entity {
                            drop_move()
                        } else {
                            pick_move()
                        }
                    }
                    _ => (
                        parent_transform.translation.x + OFFSET_X,
                        parent_transform.translation.y + OFFSET_Y,
                        parent_transform.translation.z,
                    ),
                },
            );
        }
    });
    query
        .q0_mut()
        .iter_mut()
        .for_each(|(item_entity, mut transform)| {
            if let Some(vec) = map.get(&item_entity) {
                transform.translation = Vec3::new(vec.0, vec.1, vec.2);
            }
        });
}
