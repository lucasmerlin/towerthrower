use std::f32::consts::FRAC_PI_2;

use crate::collision_sounds::CollisionSound;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::consts::BLOCK_COLLISION_GROUP;
use crate::effect::EffectType;
use crate::floor::Floor;
use crate::level::{Level, LevelLifecycle};
use crate::state::LevelState;
use crate::throw::TargetIndicator;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                rotate_aimed_blocks,
                falling_block_collision,
                block_stable_system,
                despawn_dropped_blocks,
            )
                .run_if(in_state(LevelState::Playing)),
        )
        .add_event::<CaughtBlock>();
    }
}

#[derive(Resource, Debug)]
pub struct SpawnTimer(pub Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

/// tetris like block
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlockType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub const BLOCK_SIZE: f32 = 1.0;

pub const BLOCKS: [BlockType; 7] = [
    BlockType::I,
    BlockType::O,
    BlockType::T,
    BlockType::S,
    BlockType::Z,
    BlockType::J,
    BlockType::L,
];

impl BlockType {
    pub fn random() -> Self {
        //return Self::J;
        let mut rng = rand::thread_rng();
        BLOCKS[rng.gen_range(0..BLOCKS.len())]
    }

    pub fn get_shape(&self) -> Vec<Vec2> {
        match self {
            BlockType::I => vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(2.0, 0.0),
                Vec2::new(3.0, 0.0),
            ],
            BlockType::O => vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 1.0),
            ],
            BlockType::T => vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(2.0, 0.0),
                Vec2::new(1.0, 1.0),
            ],
            BlockType::S => vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(2.0, 1.0),
            ],
            BlockType::Z => vec![
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(2.0, 0.0),
            ],
            BlockType::J => vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(2.0, 1.0),
            ],
            BlockType::L => vec![
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(2.0, 1.0),
                Vec2::new(2.0, 0.0),
            ],
        }
    }

    pub fn get_center(&self) -> Vec2 {
        match self {
            BlockType::I => Vec2::new(1.5, 0.5),
            BlockType::O => Vec2::new(0.5, 0.5),
            BlockType::T => Vec2::new(1.0, 0.5),
            BlockType::S => Vec2::new(1.0, 0.5),
            BlockType::Z => Vec2::new(1.0, 0.5),
            BlockType::J => Vec2::new(1.0, 0.5),
            BlockType::L => Vec2::new(1.0, 0.5),
        }
    }

    pub fn build_collider(&self) -> Collider {
        let size = BLOCK_SIZE;
        let half_size = size / 2.0;

        match self {
            BlockType::I => Collider::cuboid(half_size * 4.0, half_size),
            BlockType::O => Collider::cuboid(half_size * 2.0, half_size * 2.0),
            BlockType::T => Collider::compound(vec![
                (
                    Vec2::new(0.0, -half_size),
                    0.0,
                    Collider::cuboid(half_size * 3.0, half_size),
                ),
                (
                    Vec2::new(0.0, half_size),
                    0.0,
                    Collider::cuboid(half_size, half_size),
                ),
            ]),
            BlockType::S => Collider::compound(vec![
                (
                    Vec2::new(half_size, half_size),
                    0.0,
                    Collider::cuboid(half_size * 2.0, half_size),
                ),
                (
                    Vec2::new(-half_size, -half_size),
                    0.0,
                    Collider::cuboid(half_size * 2.0, half_size),
                ),
            ]),
            BlockType::Z => Collider::compound(vec![
                (
                    Vec2::new(-half_size, half_size),
                    0.0,
                    Collider::cuboid(half_size * 2.0, half_size),
                ),
                (
                    Vec2::new(half_size, -half_size),
                    0.0,
                    Collider::cuboid(half_size * 2.0, half_size),
                ),
            ]),
            BlockType::J => Collider::compound(vec![
                (
                    Vec2::new(0.0, -half_size),
                    0.0,
                    Collider::cuboid(half_size * 3.0, half_size),
                ),
                (
                    Vec2::new(-1.0 * size, half_size),
                    0.0,
                    Collider::cuboid(half_size, half_size),
                ),
            ]),
            BlockType::L => Collider::compound(vec![
                (
                    Vec2::new(0.0, -half_size),
                    0.0,
                    Collider::cuboid(half_size * 3.0, half_size),
                ),
                (
                    Vec2::new(1.0 * size, half_size),
                    0.0,
                    Collider::cuboid(half_size, half_size),
                ),
            ]),
        }
    }

    pub fn block_width(&self) -> f32 {
        match self {
            BlockType::I => 4.0,
            BlockType::O => 2.0,
            BlockType::T => 3.0,
            BlockType::S => 3.0,
            BlockType::Z => 3.0,
            BlockType::J => 3.0,
            BlockType::L => 3.0,
        }
    }

    pub fn block_height(&self) -> f32 {
        match self {
            BlockType::I => 1.0,
            BlockType::O => 2.0,
            BlockType::T => 2.0,
            BlockType::S => 2.0,
            BlockType::Z => 2.0,
            BlockType::J => 2.0,
            BlockType::L => 2.0,
        }
    }

    pub fn width(&self) -> f32 {
        self.block_width() * BLOCK_SIZE
    }

    pub fn height(&self) -> f32 {
        self.block_height() * BLOCK_SIZE
    }

    pub fn letter(&self) -> &'static str {
        match self {
            BlockType::I => "I",
            BlockType::O => "O",
            BlockType::T => "T",
            BlockType::S => "S",
            BlockType::Z => "Z",
            BlockType::J => "J",
            BlockType::L => "L",
        }
    }
}

#[derive(Component, Debug)]
pub struct Block {
    pub block_type: BlockType,
    pub effect_type: Option<EffectType>,
    pub variant: usize,
    pub initial_rotation: f32,
}

#[derive(Component)]
pub struct Falling;

#[derive(Component)]
pub struct TargetBeam;

// This is a marker component that is set while the block is being aimed
#[derive(Component)]
pub struct Aiming;

/// Marker component that will cause blocks to be destroyed on contact
#[derive(Component)]
pub struct DestroyBlockOnContact;

/// Event that is fired when a block hits some other object and quits the falling state
#[derive(Event)]
pub struct CaughtBlock {
    pub entity: Entity,
}

#[derive(Event)]
pub struct FallingBlockCollision {
    pub falling: Entity,
    pub hit: Entity,
}

impl Block {
    pub fn new(
        block_type: BlockType,
        variant: usize,
        effect_type: Option<EffectType>,
        initial_rotation: f32,
    ) -> Self {
        Self {
            block_type,
            variant,
            effect_type,
            initial_rotation,
        }
    }

    pub fn sprite(&self, assets: &AssetServer) -> SpriteBundle {
        SpriteBundle {
            texture: assets.load(format!(
                "blocks/{}/{}.png",
                self.block_type.letter(),
                self.variant,
            )),
            sprite: Sprite {
                custom_size: Some(Vec2::new(self.block_type.width(), self.block_type.height())),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn spawn(
        self,
        mut commands: &mut Commands,
        position: Vec2,
        assets: &AssetServer,
        level: &Level,
    ) -> Entity {
        let effect_type = self.effect_type;
        let block_type = self.block_type;
        let sprite = self.sprite(assets);
        let initial_rotation = self.initial_rotation;
        let entity = commands
            .spawn((
                self,
                LevelLifecycle,
                CollisionSound {
                    sound: "concrete.wav",
                    weight: 1.0,
                    ..Default::default()
                },
                //ContactForceEventThreshold(100.0),
                SpriteBundle {
                    transform: Transform::from_xyz(position.x, position.y, 0.0)
                        .with_rotation(Quat::from_rotation_z(initial_rotation)),
                    // While we're aiming the block it should be hidden
                    visibility: Visibility::Hidden,
                    ..sprite
                },
                RigidBody::KinematicVelocityBased,
                block_type.build_collider(),
                ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS,
                //ColliderMassProperties::Mass(1.0),
                ReadMassProperties::default(),
                Friction::coefficient(level.friction),
                Aiming,
                Velocity::linear(Vec2::new(0.0, -0.0)),
                Sensor,
                Sleeping::disabled(),
                // Sleeping {
                //     linear_threshold: 10.0,
                //     angular_threshold: 10.0,
                //     sleeping: false,
                // },
                ExternalImpulse::default(),
                CollisionGroups {
                    memberships: BLOCK_COLLISION_GROUP,
                    filters: Group::ALL,
                },
            ))
            .id();

        if let Some(effect) = effect_type {
            effect.enable(&mut commands, assets, entity, block_type);
        }
        entity
    }
}

pub fn falling_block_collision(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &mut Block, &mut Velocity, &mut Transform, &Falling)>,
    mut catcher_query: Query<
        (Entity),
        (Without<Falling>, Without<Aiming>, Without<TargetIndicator>),
    >,
    mut falling_block_collision: EventWriter<FallingBlockCollision>,
) {
    for event in event_reader.iter() {
        match event {
            CollisionEvent::Started(collider1, collider2, flags) => {
                [(collider1, collider2), (collider2, collider1)]
                    .into_iter()
                    .for_each(|(entity, catcher)| {
                        let mut valid_catcher = catcher_query.get_mut(*catcher);
                        if valid_catcher.is_err() {
                            return;
                        }

                        if let Ok((entity, mut block, mut velocity, mut transform, _)) =
                            query.get_mut(*entity)
                        {
                            falling_block_collision.send(FallingBlockCollision {
                                falling: entity,
                                hit: *catcher,
                            });
                        }
                    });
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

pub fn block_stable_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity), With<Falling>>,
    mut event: EventWriter<CaughtBlock>,
) {
    for (entity, velocity) in query.iter_mut() {
        if velocity.linvel.length() < 0.1 && velocity.angvel.abs() < 0.1 {
            event.send(CaughtBlock { entity });
            commands.entity(entity).remove::<Falling>();
        }
    }
}

pub fn rotate_aimed_blocks(
    mut query: Query<(Entity, &mut Transform, With<Block>, With<Aiming>)>,
    key_code: Res<Input<KeyCode>>,
    mouse_button: Res<Input<MouseButton>>,
    touch: Res<Touches>,
) {
    if mouse_button.just_pressed(MouseButton::Right)
        || (touch.iter().count() == 2 && touch.any_just_pressed())
    {
        for (entity, mut transform, ..) in query.iter_mut() {
            let vec3 = transform.translation;
            transform.rotate_around(vec3, Quat::from_rotation_z(FRAC_PI_2));
        }
    }
}

pub fn despawn_dropped_blocks(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, With<Block>)>,
    mut floor_query: Query<(&Transform, With<Floor>)>,
) {
    let (floor_transform, _) = floor_query.single();
    for (entity, mut transform, ..) in query.iter_mut() {
        if transform.translation.y < floor_transform.translation.y {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn despawn_target_beam(
    mut events: EventReader<CaughtBlock>,
    mut commands: Commands,
    query: Query<(&Children)>,
    children_query: Query<(Entity, With<TargetBeam>)>,
) {
    for event in events.iter() {
        let children = query.get(event.entity);
        if let Ok(children) = children {
            for child in children.iter() {
                if let Ok((entity, ..)) = children_query.get(*child) {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
