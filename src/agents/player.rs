use crate::plugins::{
    items::Owner,
    world::{
        AddItemToWorldEvent, GameWorld, OldPosition, Position, RemoveItemFromWorldEvent,
        YZ_PROJECTION_RATIO, TextureOffset,
    },
};

use super::SPEED;
use benimator::{AnimationPlugin, Play, SpriteSheetAnimation, SpriteSheetAnimationState};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Default)]
struct InputCommands {}

#[derive(Default, Clone)]
struct AnimationHandles {
    idle: Handle<SpriteSheetAnimation>,
    walk: Handle<SpriteSheetAnimation>,
    pick: Handle<SpriteSheetAnimation>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Walk((i32, i32)),
    Pick(Entity),
    Drop,
    PickAndDrop(Entity),
}

// pub struct Player {
//     pub state: PlayerState,
//     pub right_hand: Option<Entity>,
// }

pub struct RightHand(Option<Entity>);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub state: PlayerState,
    pub right_hand: RightHand,
    pub pos: Position,
    pub old_pos: OldPosition,
    pub offset: TextureOffset,
    pub action_timer: Timer,
}

pub struct PlayerPlugin;

const PLAYER_LAYER: f32 = 10.0;
const PLAYER_Y_OFFSET: f32 = 28.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AnimationHandles>()
            .add_startup_system(setup.system())
            .add_plugin(AnimationPlugin)
            .add_system(end_action_process.system().label("end_action_process"))
            .add_system(input.system().label("input").after("end_action_process"))
            .add_system(start_action_process.system().after("input"));
    }
}

fn setup(
    mut commands: Commands,
    mut animation_handles: ResMut<AnimationHandles>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    animation_handles.idle = animations.add(SpriteSheetAnimation::from_range(
        0..=1,
        Duration::from_millis(200),
    ));
    animation_handles.walk = animations.add(SpriteSheetAnimation::from_range(
        1..=4,
        Duration::from_millis(200),
    ));
    animation_handles.pick = animations.add(SpriteSheetAnimation::from_range(
        5..=11,
        Duration::from_millis(100),
    ));

    let texture: Handle<Texture> = asset_server.get_handle("sprites/player.png");
    let player_bundle = PlayerBundle {
        state: PlayerState::Idle,
        right_hand: RightHand(None),
        pos: Position { x: 0, y: 0 },
        old_pos: OldPosition { x: 0, y: 0 },
        offset: TextureOffset { x: 0.0, y: PLAYER_Y_OFFSET },
        action_timer: Timer::new(Default::default(), false),
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                texture,
                Vec2::new(32.0, 64.0),
                11,
                1,
            )),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
                rotation: Quat::from_rotation_x(0.0),
            },
            ..Default::default()
        })
        .insert(animation_handles.idle.clone())
        .insert(Play)
        .insert_bundle(player_bundle);
}

fn end_action_process(
    mut commands: Commands,
    mut add_event_writer: EventWriter<AddItemToWorldEvent>,
    time: Res<Time>,
    mut query: Query<(&mut PlayerState, &mut RightHand, &mut Timer, &Position, &mut OldPosition)>,
) {
    query
        .iter_mut()
        .for_each(|(mut state, mut right_hand, mut timer, pos, mut old_pos)| {
            timer.tick(time.delta());
            if !timer.finished() {
                return;
            }
            // action end process
            match *state {
                PlayerState::Idle => (),
                PlayerState::Walk(_) => {
                    old_pos.x = pos.x;
                    old_pos.y = pos.y;
                }
                PlayerState::Pick(_) => (),
                PlayerState::Drop => {
                    if let Some(item_entity) = right_hand.0 {
                        commands.entity(item_entity).remove::<Owner>().insert(*pos);
                        add_event_writer.send(AddItemToWorldEvent(item_entity, *pos));
                        right_hand.0 = None;
                    }
                }
                PlayerState::PickAndDrop(item_entity_ground) => {
                    if let Some(item_entity_hand) = right_hand.0 {
                        commands
                            .entity(item_entity_hand)
                            .remove::<Owner>()
                            .insert(*pos);
                        add_event_writer.send(AddItemToWorldEvent(item_entity_hand, *pos));
                        right_hand.0 = Some(item_entity_ground);
                    }
                }
            }
            *state = PlayerState::Idle;
        });
}

fn input(
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerState, &mut RightHand, &Timer, &Position)>,
    world_query: Query<(&GameWorld,)>,
) {
    query.iter_mut().for_each(|(mut state, mut right_hand, timer, pos)| {
        if !timer.finished() {
            return;
        }
        if key_input.pressed(KeyCode::Z) {
            for (world,) in world_query.iter() {
                if let Some(item_entity_ground) = world.item_map.get(&pos) {
                    if let Some(_) = right_hand.0 {
                        *state = PlayerState::PickAndDrop(*item_entity_ground);
                    } else {
                        *state = PlayerState::Pick(*item_entity_ground);
                    }
                } else {
                    if let Some(_) = right_hand.0 {
                        *state = PlayerState::Drop;
                    }
                }
            }
        }
        if *state == PlayerState::Idle {
            let mut walk = (0, 0);
            if key_input.pressed(KeyCode::Left) {
                walk.0 -= SPEED;
            }
            if key_input.pressed(KeyCode::Right) {
                walk.0 += SPEED;
            }
            if key_input.pressed(KeyCode::Down) {
                walk.1 -= SPEED;
            }
            if key_input.pressed(KeyCode::Up) {
                walk.1 += SPEED;
            }
            if walk != (0, 0) {
                *state = PlayerState::Walk(walk);
            }
        }
    });
}

fn start_action_process(
    mut commands: Commands,
    mut remove_event_writer: EventWriter<RemoveItemFromWorldEvent>,
    animation_handles: Res<AnimationHandles>,
    mut query: Query<(
        Entity,
        &mut PlayerState,
        &mut RightHand,
        &mut Position,
        &mut OldPosition,
        &mut Timer,
        &mut Handle<SpriteSheetAnimation>,
        &mut SpriteSheetAnimationState,
    )>,
) {
    query.iter_mut().for_each(
        |(entity, state, mut right_hand, mut pos, mut old_pos, mut timer, mut handle, mut animation_state)| {
            if !timer.finished() {
                return;
            }
            match *state {
                PlayerState::Idle => {
                    *handle = animation_handles.idle.clone();
                }
                PlayerState::Walk((x, y)) => {
                    *handle = animation_handles.walk.clone();
                    timer.set_duration(std::time::Duration::from_millis(200));
                    timer.reset();

                    let old = pos.clone();

                    pos.x += x;
                    pos.y += y;

                    old_pos.x = old.x;
                    old_pos.y = old.y;
                }
                PlayerState::Pick(item_entity) => {
                    *handle = animation_handles.pick.clone();
                    timer.set_duration(std::time::Duration::from_millis(500));
                    timer.reset();
                    animation_state.reset();

                    right_hand.0 = Some(item_entity);
                    commands
                        .entity(item_entity)
                        .remove::<Position>()
                        .insert(Owner(entity));
                    remove_event_writer.send(RemoveItemFromWorldEvent(item_entity, *pos));
                }
                PlayerState::Drop => {
                    *handle = animation_handles.pick.clone();
                    timer.set_duration(std::time::Duration::from_millis(500));
                    timer.reset();
                    animation_state.reset();
                }
                PlayerState::PickAndDrop(item_entity_ground) => {
                    *handle = animation_handles.pick.clone();
                    timer.set_duration(std::time::Duration::from_millis(500));
                    timer.reset();
                    animation_state.reset();
                    commands
                        .entity(item_entity_ground)
                        .remove::<Position>()
                        .insert(Owner(entity));
                    remove_event_writer.send(RemoveItemFromWorldEvent(item_entity_ground, *pos));
                }
            }
        },
    );
}
