use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{random, thread_rng, Rng};

use crate::block::{DestroyBlockOnContact, BLOCK_COLLISION_GROUP};
use crate::floor::FLOOR_COLLISION_GROUP;
use crate::level::{LevelLifecycle, UpdateLevelStats};
use crate::{CAR_MAX_HEIGHT, CAR_MIN_HEIGHT, HORIZONTAL_VIEWPORT_SIZE};

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

#[derive(Component, Debug, Default)]
pub struct Car {
    state: CarState,
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
            spawn_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

pub fn spawn_car_system(
    mut commands: Commands,
    mut car_spawner: ResMut<CarSpawner>,
    time: Res<Time>,
) {
    car_spawner.spawn_timer.tick(time.delta());
    if car_spawner.spawn_timer.just_finished() {
        let car_size = Vec2::new(
            thread_rng().gen_range(3.0..5.0),
            thread_rng().gen_range(1.5..2.0),
        );

        let direction = if random() { 1.0 } else { -1.0 };

        let car_position = Vec2::new(
            direction * HORIZONTAL_VIEWPORT_SIZE,
            thread_rng().gen_range(CAR_MIN_HEIGHT..CAR_MAX_HEIGHT),
        );

        let car_velocity = Vec2::new(direction * -thread_rng().gen_range(7.0..10.0), 0.0);

        commands.spawn((
            Car::default(),
            LevelLifecycle,
            DestroyBlockOnContact,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(car_size),
                    color: Color::rgb(0.3, 0.3, 0.3),
                    ..Default::default()
                },
                transform: Transform::from_xyz(car_position.x, car_position.y, 0.0),
                ..Default::default()
            },
            RigidBody::KinematicVelocityBased,
            Collider::cuboid(car_size.x / 2.0, car_size.y / 2.0),
            Friction::coefficient(0.5),
            Velocity::linear(car_velocity),
        ));
        car_spawner.spawn_timer =
            Timer::from_seconds(random::<f32>() * 3.0 + 3.0, TimerMode::Repeating);
    }
}

pub fn car_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut car_query: Query<(Entity, &mut Car)>,
    mut car_crashed_events: EventWriter<CarCrashedEvent>,
    mut update_level_stats_events: EventWriter<UpdateLevelStats>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(a, b, _) => [a, b].into_iter().for_each(|entity| {
                if let Ok((entity, mut car)) = car_query.get_mut(*entity) {
                    if !matches!(car.state, CarState::Driving) {
                        return;
                    }
                    car.state = CarState::Crashed {
                        remove_debris_collision_timer: Timer::from_seconds(0.5, TimerMode::Once),
                    };
                    commands.entity(entity).insert((RigidBody::Dynamic,));
                    car_crashed_events.send(CarCrashedEvent { entity });
                    update_level_stats_events.send(UpdateLevelStats::CarHit);
                }
            }),
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
