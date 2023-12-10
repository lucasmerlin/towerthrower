use crate::effect::glue::GlueEffect;
use crate::effect::platform::PlatformEffect;
use bevy::prelude::*;

pub mod glue;
pub mod magnetic;
pub mod platform;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            glue::GluePlugin,
            platform::PlatformEffectPlugin,
            magnetic::MagneticPlugin,
        ));
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    Glue,
    Platform,
    Magnetic,
}

pub const ALL_EFFECTS: [EffectType; 3] =
    [EffectType::Glue, EffectType::Platform, EffectType::Magnetic];

pub const DEFAULT_EFFECTS: [EffectType; 2] = [EffectType::Glue, EffectType::Magnetic];

impl EffectType {
    pub fn texture(&self) -> &'static str {
        match self {
            EffectType::Glue => "glue.png",
            EffectType::Platform => "fixed.png",
            EffectType::Magnetic => "magnet.png",
        }
    }

    fn insert_effect(&self, commands: &mut Commands, entity: Entity) {
        match self {
            EffectType::Glue => {
                commands.entity(entity).insert(GlueEffect::default());
            }
            EffectType::Platform => {
                commands.entity(entity).insert(PlatformEffect::default());
            }
            EffectType::Magnetic => {
                commands
                    .entity(entity)
                    .insert(magnetic::MagneticEffect::default());
            }
        }
    }

    pub fn enable(&self, commands: &mut Commands, assets: &mut AssetServer, entity: Entity) {
        self.insert_effect(commands, entity);
        let texture = self.texture();
        commands.entity(entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: assets.load(texture),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
}
