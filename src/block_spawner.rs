use bevy::prelude::*;

use crate::block::*;
use crate::camera_movement::CameraMovement;

#[derive(Resource)]
pub struct BlockSpawner {
    current_blocks: Vec<Entity>,
    target_count: usize,
    max_spawn_rate: Timer,
}

impl Default for BlockSpawner {
    fn default() -> Self {
        Self {
            current_blocks: vec![],
            target_count: 1,
            max_spawn_rate: Timer::from_seconds(0.25, TimerMode::Once),
        }
    }
}

pub fn spawn_missing_blocks(
    mut spawner: ResMut<BlockSpawner>,
    camera_movement: Res<CameraMovement>,
    mut commands: Commands,
    mut assets: ResMut<AssetServer>,
    mut time: ResMut<Time>,
) {
    spawner.max_spawn_rate.tick(time.delta());
    let current_count = spawner.current_blocks.len();
    if current_count < spawner.target_count {
        if spawner.max_spawn_rate.finished() {
            spawner.max_spawn_rate.reset();
            let block_type = BlockType::random();
            let area = 200.0;
            let position = Vec2::new(
                rand::random::<f32>() * area - area / 2.0,
                camera_movement.height + 200.0,
            );
            let block = Block::spawn(&mut commands, block_type, position, &mut assets);
            spawner.current_blocks.push(block);
        }
    }
}

pub fn remove_not_falling_blocks(
    mut spawner: ResMut<BlockSpawner>,
    mut commands: Commands,
    query: Query<(Entity, &Block), Without<Falling>>,
) {
    for (entity, block) in query.iter() {
        spawner.current_blocks.retain(|&x| x != entity);
    }
}

pub fn untrack_despawned_blocks(
    mut spawner: ResMut<BlockSpawner>,
    mut commands: Commands,
    query: Query<(Entity, &Block)>,
) {
    spawner.current_blocks.retain(|&x| match query.get(x) {
        Ok(ok) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}
