use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::prelude::*;
use rand::prelude::SliceRandom;
use rand::{random, thread_rng, Rng};

use crate::block::{Block, DestroyBlockOnContact};
use crate::collision_sounds::CollisionSound;
use crate::consts::{BLOCK_COLLISION_GROUP, FLOOR_COLLISION_GROUP};
use crate::environment::rain::DarkenSpriteOnRain;
use crate::level::{LevelLifecycle, UpdateLevelStats};
use crate::{
    ASSET_SCALE, CAR_MAX_HEIGHT, CAR_MIN_HEIGHT, CAR_RATE, CAR_SCALE, HORIZONTAL_VIEWPORT_SIZE,
};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_car_system,
                car_collision_system,
                car_remove_debris_collision_timer_system,
            ),
        )
        .init_resource::<CarSpawner>()
        .add_event::<CarCrashedEvent>();
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CarType {
    Beetle,
    Blue,
    Bully1,
    Bully2,
    Lila,
    Red,
}

pub const CAR_TYPES: [CarType; 6] = [
    CarType::Beetle,
    CarType::Blue,
    CarType::Bully1,
    CarType::Bully2,
    CarType::Lila,
    CarType::Red,
];

impl CarType {
    pub fn off_texture_size(&self) -> Vec2 {
        match self {
            CarType::Beetle => Vec2::new(487.0, 210.0),
            CarType::Blue => Vec2::new(500.0, 180.0),
            CarType::Bully1 => Vec2::new(580.0, 289.0),
            CarType::Bully2 => Vec2::new(570.0, 260.0),
            CarType::Lila => Vec2::new(500.0, 180.0),
            CarType::Red => Vec2::new(500.0, 211.0),
        }
    }

    pub fn on_texture_height(&self) -> f32 {
        match self {
            CarType::Beetle => 400.0,
            CarType::Blue => 357.0,
            CarType::Bully1 => 459.0,
            CarType::Bully2 => 417.0,
            CarType::Lila => 351.0,
            CarType::Red => 405.0,
        }
    }

    pub fn off_size(&self) -> Vec2 {
        self.off_texture_size() * ASSET_SCALE * CAR_SCALE
    }

    pub fn on_size(&self) -> Vec2 {
        Vec2::new(
            self.off_texture_size().x * ASSET_SCALE * CAR_SCALE,
            self.on_texture_height() * ASSET_SCALE * CAR_SCALE,
        )
    }

    pub fn name(&self) -> &str {
        match self {
            CarType::Beetle => "beetle",
            CarType::Blue => "blue",
            CarType::Bully1 => "bully_1",
            CarType::Bully2 => "bully_2",
            CarType::Lila => "lila",
            CarType::Red => "red",
        }
    }

    pub fn on_asset_path(&self) -> String {
        format!("cars/{}/on.png", self.name())
    }

    pub fn off_asset_path(&self) -> String {
        format!("cars/{}/off.png", self.name())
    }
}

#[derive(Component, Debug)]
pub struct Car {
    state: CarState,
    car_type: CarType,
}

#[derive(Component, Debug)]
struct CarTexture {
    on: bool,
}

#[derive(Debug, Default)]
pub enum CarState {
    #[default]
    Driving,
    Crashed {
        remove_debris_collision_timer: Timer,
    },
}

#[derive(Event, Debug)]
pub struct CarCrashedEvent {
    pub entity: Entity,
}

#[derive(Resource, Debug)]
pub struct CarSpawner {
    pub spawn_timer: Timer,
}

impl Default for CarSpawner {
    fn default() -> Self {
        Self {
            spawn_timer: Timer::from_seconds(CAR_RATE, TimerMode::Repeating),
        }
    }
}

pub fn spawn_car_system(
    mut commands: Commands,
    mut car_spawner: ResMut<CarSpawner>,
    time: Res<Time>,
    assets: Res<AssetServer>,
) {
    car_spawner.spawn_timer.tick(time.delta());
    if car_spawner.spawn_timer.just_finished() {
        let car_type = CAR_TYPES.choose(&mut thread_rng()).unwrap();

        let car_size = car_type.off_size();

        let forward = random::<bool>();

        let direction = if forward { 1.0 } else { -1.0 };

        let car_position = Vec2::new(
            direction * HORIZONTAL_VIEWPORT_SIZE,
            thread_rng().gen_range(CAR_MIN_HEIGHT..CAR_MAX_HEIGHT),
        );

        let car_velocity = Vec2::new(direction * -thread_rng().gen_range(7.0..10.0), 0.0);

        commands
            .spawn((
                Car {
                    state: CarState::Driving,
                    car_type: *car_type,
                },
                CollisionSound {
                    sound: "car_crash.wav",
                    weight: 2.0,
                    ..Default::default()
                },
                LevelLifecycle,
                DestroyBlockOnContact,
                SpatialBundle::from_transform(Transform::from_xyz(
                    car_position.x,
                    car_position.y,
                    0.0,
                )),
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(car_size.x / 2.0, car_size.y / 2.0),
                Friction::coefficient(0.5),
                Velocity::linear(car_velocity),
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(car_type.on_size()),
                            flip_x: !forward,
                            anchor: Anchor::TopCenter,
                            ..Default::default()
                        },
                        texture: assets.load(car_type.on_asset_path()),
                        transform: Transform::from_xyz(0.0, car_size.y / 2.0, 0.0),
                        ..Default::default()
                    },
                    CarTexture { on: true },
                    DarkenSpriteOnRain(0.8),
                ));
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(car_type.off_size()),
                            flip_x: !forward,
                            anchor: Anchor::TopCenter,
                            ..Default::default()
                        },
                        texture: assets.load(car_type.off_asset_path()),
                        transform: Transform::from_xyz(0.0, car_size.y / 2.0, 0.0),
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                    CarTexture { on: false },
                    DarkenSpriteOnRain(0.8),
                ));
            });
        car_spawner.spawn_timer =
            Timer::from_seconds(random::<f32>() * CAR_RATE + CAR_RATE, TimerMode::Repeating);
    }
}

fn car_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut car_query: Query<(Entity, &mut Car, &Children)>,
    mut car_sprite_query: Query<(&CarTexture)>,
    mut block_query: Query<Entity, With<Block>>,
    mut car_crashed_events: EventWriter<CarCrashedEvent>,
    mut update_level_stats_events: EventWriter<UpdateLevelStats>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                [(a, b), (b, a)]
                    .into_iter()
                    .for_each(|(car_entity, block_entity)| {
                        if !block_query.get(*block_entity).is_ok() {
                            return;
                        }
                        if let Ok((car_entity, mut car, children)) = car_query.get_mut(*car_entity)
                        {
                            if !matches!(car.state, CarState::Driving) {
                                return;
                            }
                            car.state = CarState::Crashed {
                                remove_debris_collision_timer: Timer::from_seconds(
                                    0.5,
                                    TimerMode::Once,
                                ),
                            };
                            commands.entity(car_entity).insert((RigidBody::Dynamic,));
                            car_crashed_events.send(CarCrashedEvent { entity: car_entity });
                            update_level_stats_events.send(UpdateLevelStats::CarHit);

                            for child in children.iter() {
                                if let Ok((texture)) = car_sprite_query.get_mut(*child) {
                                    if texture.on {
                                        commands.entity(*child).insert(Visibility::Hidden);
                                    } else {
                                        commands.entity(*child).insert(Visibility::Visible);
                                    }
                                }
                            }
                        }
                    })
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

pub fn car_remove_debris_collision_timer_system(
    mut commands: Commands,
    mut car_query: Query<(Entity, &mut Car)>,
    mut time: ResMut<Time>,
) {
    for (entity, mut car) in car_query.iter_mut() {
        if let CarState::Crashed {
            remove_debris_collision_timer,
        } = &mut car.state
        {
            remove_debris_collision_timer.tick(time.delta());
            if remove_debris_collision_timer.just_finished() {
                commands.entity(entity).insert(
                    (CollisionGroups {
                        filters: FLOOR_COLLISION_GROUP | BLOCK_COLLISION_GROUP,
                        memberships: Group::default(),
                    }),
                );
            }
        }
    }
}
