use bevy::audio::{Volume, VolumeLevel};
use bevy::prelude::*;
use rand::random;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_music);
    }
}

#[derive(Component, Debug)]
struct Music;

const SONGS: [&str; 6] = [
    "Crinoline Dreams.mp3",
    "Faster Does It.mp3",
    "Intractable.mp3",
    "Off to Osaka.mp3",
    "Rollin at 5.mp3",
    "Vibing Over Venus.mp3",
];

fn setup_music(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut index: Local<Option<usize>>,
    current: Query<&Music>,
) {
    if index.is_none() {
        *index = Some(random::<usize>() % SONGS.len());
    }

    if current.iter().next().is_some() {
        return;
    }

    let index = index.as_mut().unwrap();
    *index += 1;
    if *index >= SONGS.len() {
        *index = 0;
    }

    commands.spawn((
        AudioBundle {
            source: assets.load(format!("music/{}", SONGS[*index])),
            settings: PlaybackSettings {
                volume: Volume::Relative(VolumeLevel::new(0.5)),
                ..PlaybackSettings::DESPAWN
            },
        },
        Music,
    ));
}
