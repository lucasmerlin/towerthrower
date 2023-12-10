use crate::level::LevelLifecycle;
use crate::CAR_SCALE;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct BeamPlugin;

impl Plugin for BeamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_beam_system, update_beam_system))
            .add_event::<BeamEvent>();
    }
}

#[derive(Component, Debug)]
pub struct Beam {
    pub target: Entity,
}

#[derive(Bundle, Debug)]
pub struct BeamBundle {
    pub beam: Beam,
    pub source_transform: SpatialBundle,
}

#[derive(Event, Debug)]
pub struct BeamEvent {
    pub source: Entity,
    pub target: Entity,
    pub source_offset: Vec3,
}

pub fn spawn_beam_system(mut commands: Commands, mut beam_events: EventReader<BeamEvent>) {
    for event in beam_events.read() {
        commands.entity(event.source).with_children(|parent| {
            parent.spawn((
                Beam {
                    target: event.target,
                },
                LevelLifecycle,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba_u8(255, 255, 255, 200),
                        custom_size: Some(Vec2::new(0.0, 0.0)),
                        anchor: Anchor::CenterLeft,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(event.source_offset),
                    ..Default::default()
                },
            ));
        });
    }
}

pub fn update_beam_system(
    mut query: Query<(Entity, &Beam, &mut Transform, &GlobalTransform, &mut Sprite)>,
    target_query: Query<&GlobalTransform>,
) {
    for (beam_entity, beam, mut beam_transform, beam_global_transform, mut sprite) in
        query.iter_mut()
    {
        if let Ok(target_global_transform) = target_query.get(beam.target) {
            let direction =
                target_global_transform.translation() - beam_global_transform.translation();
            beam_transform.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x));

            let beam_length = (target_global_transform.translation()
                - beam_global_transform.translation())
            .length();

            sprite.custom_size = Some(Vec2::new(beam_length, 0.5 * CAR_SCALE));
        }
    }
}
