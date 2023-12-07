use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::block::{Aiming, Block, Falling};
use crate::state::LevelState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                load_level_event,
                (check_current_block_stats, check_win_condition)
                    .run_if(in_state(LevelState::Playing)),
            ),
        )
        .add_systems(OnEnter(LevelState::Playing), reset_level_stats)
        .init_resource::<LevelStats>()
        .insert_resource(LEVELS[0].clone())
        .add_event::<NextLevel>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct LevelStats {
    pub current_height: f32,
    pub blocks_thrown: usize,
    pub current_block_count: usize,
    pub blocks_missed: usize,

    pub deaths: usize,
}

#[derive(Resource, Debug, Clone)]
pub struct Level {
    pub level: usize,
    pub goal: LevelGoal,
    pub max_blocks: Option<usize>,
    pub time_limit: Option<Duration>,
    pub base_width: f32,
}

#[derive(Debug, Clone)]
pub enum LevelGoal {
    ReachHeight(f32),
    ReachBlockCount(usize),
}

#[derive(Event, Debug, Clone)]
pub struct NextLevel(pub Option<usize>);

#[derive(Component, Debug, Clone)]
pub struct LevelLifecycle;

pub static LEVELS: [Level; 4] = [
    Level {
        level: 0,
        goal: LevelGoal::ReachHeight(200.0),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: None,
        base_width: 160.0,
    },
    Level {
        level: 1,
        goal: LevelGoal::ReachBlockCount(10),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: None,
        base_width: 80.0,
    },
    Level {
        level: 2,
        goal: LevelGoal::ReachHeight(200.0),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: None,
        base_width: 120.0,
    },
    Level {
        level: 3,
        goal: LevelGoal::ReachBlockCount(10),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: None,
        base_width: 160.0,
    },
];

pub fn load_level_event(
    mut commands: Commands,
    mut evr: EventReader<NextLevel>,
    mut level_stats: ResMut<LevelStats>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut level_res: ResMut<Level>,
) {
    for next_level in evr.read() {
        let level = if let Some(level) = next_level.0 {
            level
        } else {
            level_res.level + 1
        };

        if let Some(next_level) = LEVELS.get(level) {
            *level_stats = LevelStats::default();

            *level_res = next_level.clone();
            level_state.set(LevelState::Loading);
        }
    }
}

pub fn check_current_block_stats(
    query: Query<(&Transform, &Velocity), (With<Block>, Without<Aiming>, Without<Falling>)>,
    mut level_stats: ResMut<LevelStats>,
) {
    let mut max_height = 0.0;
    let mut block_count = 0;

    for (transform, velocity) in query.iter() {
        if velocity.linvel.length() < 1.0 {
            block_count += 1;
            if transform.translation.y > max_height {
                max_height = transform.translation.y;
            }
        }
    }

    level_stats.current_height = max_height;
    level_stats.current_block_count = block_count;
}

pub fn check_win_condition(
    mut commands: Commands,
    level_stats: Res<LevelStats>,
    level: Res<Level>,
    mut state: ResMut<NextState<LevelState>>,
) {
    match level.goal {
        LevelGoal::ReachHeight(height) => {
            if level_stats.current_height >= height {
                state.set(LevelState::Won);
            }
        }
        LevelGoal::ReachBlockCount(count) => {
            if level_stats.current_block_count >= count {
                state.set(LevelState::Won);
            }
        }
    }
}

fn reset_level_stats(mut level_stats: ResMut<LevelStats>) {
    *level_stats = LevelStats::default();
}
