use crate::effect::glue::GlueEffect;
use crate::effect::platform::PlatformEffect;
use bevy::prelude::*;

mod glue;
mod magnetic;
mod platform;

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

#[derive(Component, Debug)]
pub struct BlockEffect(pub EffectType);

#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    Glue,
}

pub fn add_random_effect(commands: &mut Commands, assets: &mut AssetServer, entity: Entity) {
    let rand = rand::random::<u8>() % 20;
    //let rand = 2;
    let res = match rand {
        0 => Some((
            commands.entity(entity).insert(GlueEffect::default()).id(),
            "glue.png",
        )),
        1 => Some((
            commands
                .entity(entity)
                .insert(PlatformEffect::default())
                .id(),
            "fixed.png",
        )),
        2 => Some((
            commands
                .entity(entity)
                .insert(magnetic::MagneticEffect::default())
                .id(),
            "magnet-on.png",
        )),
        _ => None,
    };

    if let Some((entity, texture)) = res {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: assets.load(texture),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(24.0, 24.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
}
