use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::base::Base;
use crate::effect::add_random_effect;
use crate::throw::TargetIndicator;

#[derive(Resource, Debug)]
pub struct SpawnTimer(pub Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

/// tetris like block
#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl BlockType {
    pub fn random() -> Self {
        match rand::random::<u8>() % 7 {
            0 => BlockType::I,
            1 => BlockType::O,
            2 => BlockType::T,
            3 => BlockType::S,
            4 => BlockType::Z,
            5 => BlockType::J,
            _ => BlockType::L,
        }
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
        let size = 20.0;
        let shape = self.get_shape();
        Collider::compound(
            shape
                .iter()
                .map(|v| {
                    (
                        Vect::new(v.x * size, v.y * size) - self.get_center() * size,
                        0.0,
                        Collider::cuboid(size / 2.0, size / 2.0),
                    )
                })
                .collect(),
        )
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

    pub fn width(&self) -> f32 {
        self.block_width() * 20.0
    }
}

#[derive(Component)]
pub struct Block {
    pub block_type: BlockType,
}

#[derive(Component)]
pub struct Falling;

#[derive(Component)]
pub struct TargetBeam;

// This is a marker component that is set while the block is being aimed
#[derive(Component)]
pub struct Aiming;

/// Event that is fired when a block hits some other object and quits the falling state
#[derive(Event)]
pub struct CaughtBlock {
    pub entity: Entity,
    pub caught_by: Entity,
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }

    pub fn spawn(
        mut commands: &mut Commands,
        block_type: BlockType,
        position: Vec2,
        assets: &mut AssetServer,
    ) -> Entity {
        let block = Block::new(block_type);
        let width = block_type.width();
        let entity = commands
            .spawn((
                block,
                SpatialBundle::from(Transform::from_xyz(position.x, position.y, 0.0)),
                RigidBody::KinematicVelocityBased,
                block_type.build_collider(),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
                ColliderMassProperties::Mass(0.1),
                Friction::coefficient(0.5),
                Aiming,
                Velocity::linear(Vec2::new(0.0, -0.0)),
                Sensor,
                Sleeping::disabled(),
                ExternalImpulse::default(),
            ))
            .id();

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba_u8(203, 158, 255, 100),
                        custom_size: Some(Vec2::new(width, 100000.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                TargetBeam,
            ));
        });

        add_random_effect(&mut commands, assets, entity);
        entity
    }
}

pub fn block_collision(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &mut Block, &mut Velocity, &mut Transform, &Falling)>,
    mut catcher_query: Query<
        (Entity),
        (Without<Falling>, Without<Aiming>, Without<TargetIndicator>),
    >,
    mut stopped_falling_events: EventWriter<CaughtBlock>,
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
                            println!("Caught block {:?}", entity);
                            //*velocity = Velocity::linear(Vec2::ZERO);
                            //transform.translation.y += 1.0;
                            commands.entity(entity).remove::<Falling>();
                            //commands.entity(entity).remove::<Sensor>();
                            //commands.entity(entity).insert(RigidBody::Dynamic);

                            stopped_falling_events.send(CaughtBlock {
                                entity,
                                caught_by: *catcher,
                            });
                        }
                    });
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

pub fn rotate_falling_blocks(
    mut query: Query<(Entity, &mut Transform, With<Block>, With<Falling>)>,
    key_code: Res<Input<KeyCode>>,
) {
    if key_code.just_pressed(KeyCode::Up) || key_code.just_pressed(KeyCode::Space) {
        for (entity, mut transform, ..) in query.iter_mut() {
            let vec3 = transform.translation;
            transform.rotate_around(vec3, Quat::from_rotation_z(FRAC_PI_2));
        }
    }
}

pub fn despawn_droped_blocks(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, With<Block>)>,
    mut base_query: Query<(&Transform, With<Base>)>,
) {
    let (base_transform, _) = base_query.single();
    for (entity, mut transform, ..) in query.iter_mut() {
        if transform.translation.y < base_transform.translation.y {
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
