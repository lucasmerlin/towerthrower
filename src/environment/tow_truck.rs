use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::environment::beam::BeamEvent;
use crate::environment::car::{Car, CarCrashedEvent};

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
) {
    for event in car_crashed_events.read() {
        let size = Vec2::new(60.0, 30.0);

        commands.spawn((
            TowTruck {
                target: event.entity,
                phase: TowTruckPhase::MovingToTarget,
            },
            SpatialBundle::from(Transform::from_xyz(-1000.0, 0.0, 0.0)),
            RigidBody::KinematicVelocityBased,
            Collider::cuboid(size.x / 2.0, size.y / 2.0),
            Velocity::linear(Vec2::new(50.0, 0.0)),
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
                    let distance = tow_truck_transform
                        .translation
                        .distance(car_transform.translation);

                    if distance < 100.0 {
                        tow_truck.phase = TowTruckPhase::RaisingCar;
                        beam_event_writer.send(BeamEvent {
                            source: tow_truck_entity,
                            target: car_entity,
                            source_offset: Vec3::new(15.0, 0.0, 0.0),
                        });
                        commands
                            .entity(tow_truck_entity)
                            .add_child(tow_truck.target);

                        commands.entity(car_entity).insert((
                            RigidBody::KinematicVelocityBased,
                            Velocity::linear(Vec2::new(0.5, 15.0)),
                        ));
                        tow_truck_velocity.linvel = Vec2::ZERO;

                        car_transform.translation =
                            car_transform.translation - tow_truck_transform.translation;
                    }
                }
                TowTruckPhase::RaisingCar => {
                    if car_transform.translation.y > -30.0 {
                        tow_truck.phase = TowTruckPhase::TowingTarget;
                        commands
                            .entity(car_entity)
                            .insert((RigidBody::Fixed, Velocity::linear(Vec2::new(0.0, 0.0))));
                        car_velocity.linvel = Vec2::ZERO;

                        tow_truck_velocity.linvel = Vec2::new(50.0, 0.0);
                    }
                }
                TowTruckPhase::TowingTarget => {}
            }
        }
    }
}
