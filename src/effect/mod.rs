use bevy::prelude::*;

use crate::block::BlockType;
use crate::effect::glue::GlueEffect;
use crate::effect::platform::PlatformEffect;

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

pub fn glue_texture(block_type: BlockType) -> String {
    format!("effects/glue/{}.png", block_type.letter().to_lowercase())
}

impl EffectType {
    pub fn texture(&self, block_type: BlockType) -> String {
        match self {
            EffectType::Glue => glue_texture(block_type),
            EffectType::Platform => "fixed.png".to_string(),
            EffectType::Magnetic => {
                format!("effects/magnet/{}.png", block_type.letter().to_lowercase())
            }
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

    pub fn enable(
        &self,
        commands: &mut Commands,
        assets: &AssetServer,
        entity: Entity,
        block_type: BlockType,
    ) {
        self.insert_effect(commands, entity);
        let texture = self.texture(block_type);
        commands.entity(entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                texture: assets.load(texture),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(block_type.width(), block_type.height())),
                    color: Color::rgba(1.0, 1.0, 1.0, 0.75),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
}
