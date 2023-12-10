use bevy::audio::{Volume, VolumeLevel};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent, Velocity};

pub struct CollisionSoundPlugin;

impl Plugin for CollisionSoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_sound_system);
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CollisionSound {
    pub sound: &'static str,
    pub weight: f32,
    pub min_velocity: f32,
}

impl Default for CollisionSound {
    fn default() -> Self {
        Self {
            sound: "thud.wav",
            weight: 1.0,
            min_velocity: 2.0,
        }
    }
}

pub fn collision_sound_system(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    sound_query: Query<(&CollisionSound, &Velocity)>,
    assets: Res<AssetServer>,
) {
    for event in events.read() {
        if let CollisionEvent::Started(a, b, _) = event {
            let a_sound = sound_query.get(*a);
            let b_sound = sound_query.get(*b);

            if let (Ok((a_sound, a_velocity)), Ok((b_sound, b_velocity))) = (a_sound, b_sound) {
                let relative_velocity = (a_velocity.linvel - b_velocity.linvel).length();

                if relative_velocity < a_sound.min_velocity
                    && relative_velocity < b_sound.min_velocity
                {
                    continue;
                }

                let file = if a_sound.weight > b_sound.weight {
                    a_sound.sound
                } else {
                    b_sound.sound
                };

                let sound = assets.load(format!("sounds/{}", file));

                // volume is 0 at min_velocity, 1 at 2 * min_velocity, 2 at 3 * min_velocity
                let volume = (relative_velocity * 0.01).min(1.0);

                commands.spawn(
                    (AudioBundle {
                        source: sound,
                        settings: PlaybackSettings {
                            volume: Volume::Relative(VolumeLevel::new(volume)),
                            ..PlaybackSettings::DESPAWN
                        },
                    }),
                );
            }
        }
    }
}
