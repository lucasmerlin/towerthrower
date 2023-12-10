use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::environment::beam::BeamEvent;
use crate::environment::car::{Car, CarCrashedEvent};
use crate::level::LevelLifecycle;
use crate::{ASSET_SCALE, CAR_MAX_HEIGHT, CAR_MIN_HEIGHT, CAR_SCALE, HORIZONTAL_VIEWPORT_SIZE};

pub struct TowTruckPlugin;

impl Plugin for TowTruckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_tow_truck_system, tow_car_system));
    }
}

#[derive(Component, Debug)]
pub struct TowTruck {
    pub target: Entity,
    pub phase: TowTruckPhase,
}

#[derive(Debug, Copy, Clone)]
pub enum TowTruckPhase {
    MovingToTarget,
    RaisingCar,
    TowingTarget,
}

pub fn spawn_tow_truck_system(
    mut commands: Commands,
    mut car_crashed_events: EventReader<CarCrashedEvent>,
    assets: Res<AssetServer>,
) {
    for event in car_crashed_events.read() {
        let res_w = 581.0;
        let res_h = 462.0;

        let pos_y = thread_rng().gen_range(CAR_MIN_HEIGHT..CAR_MAX_HEIGHT);

        let size = Vec2::new(
            res_w * ASSET_SCALE * CAR_SCALE,
            res_h * ASSET_SCALE * CAR_SCALE,
        );

        commands.spawn((
            TowTruck {
                target: event.entity,
                phase: TowTruckPhase::MovingToTarget,
            },
            LevelLifecycle,
            SpriteBundle {
                transform: Transform::from_xyz(-HORIZONTAL_VIEWPORT_SIZE, pos_y, 0.0),
                texture: assets.load("cars/tow_truck.png"),
                sprite: Sprite {
                    custom_size: Some(size),
                    flip_x: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::KinematicVelocityBased,
            Collider::cuboid(size.x / 2.0, size.y / 2.0),
            Velocity::linear(Vec2::new(5.0, 0.0)),
            Sensor,
        ));
    }
}

pub fn tow_car_system(
    mut commands: Commands,
    mut tow_truck_query: Query<(Entity, &mut TowTruck, &Transform, &mut Velocity)>,
    mut car_query: Query<(Entity, &mut Car, &mut Transform, &mut Velocity), Without<TowTruck>>,
    mut beam_event_writer: EventWriter<BeamEvent>,
) {
    for (tow_truck_entity, mut tow_truck, tow_truck_transform, mut tow_truck_velocity) in
        tow_truck_query.iter_mut()
    {
        if let Ok((car_entity, mut car, mut car_transform, mut car_velocity)) =
            car_query.get_mut(tow_truck.target)
        {
            match tow_truck.phase {
                TowTruckPhase::MovingToTarget => {
                    if tow_truck_transform.translation.x > car_transform.translation.x + 7.0 {
                        tow_truck.phase = TowTruckPhase::RaisingCar;
                        beam_event_writer.send(BeamEvent {
                            source: tow_truck_entity,
                            target: car_entity,
                            source_offset: Vec3::new(-2.4, 2.3, 0.0) * CAR_SCALE,
                        });
                        commands
                            .entity(tow_truck_entity)
                            .add_child(tow_truck.target);

                        commands.entity(car_entity).insert((
                            RigidBody::KinematicVelocityBased,
                            Velocity::linear(Vec2::new(0.05, 1.5)),
                        ));
                        tow_truck_velocity.linvel = Vec2::ZERO;

                        car_transform.translation =
                            car_transform.translation - tow_truck_transform.translation;
                    }
                }
                TowTruckPhase::RaisingCar => {
                    if car_transform.translation.y > 1.6 {
                        tow_truck.phase = TowTruckPhase::TowingTarget;
                        commands
                            .entity(car_entity)
                            .insert((RigidBody::Fixed, Velocity::linear(Vec2::new(0.0, 0.0))));
                        car_velocity.linvel = Vec2::ZERO;

                        tow_truck_velocity.linvel = Vec2::new(5.0, 0.0);
                    }
                }
                TowTruckPhase::TowingTarget => {}
            }
        }
    }
}
