use std::time::Duration;

use crate::base::BaseType;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::block::{Aiming, Block, Falling};
use crate::effect::EffectType;
use crate::state::LevelState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                load_level_event,
                (
                    check_current_block_stats,
                    check_win_loose_condition,
                    update_level_stats_events,
                )
                    .run_if(in_state(LevelState::Playing)),
            ),
        )
        .add_systems(OnEnter(LevelState::Playing), reset_level_stats)
        .init_resource::<LevelStats>()
        .insert_resource(LEVELS[0].clone())
        .add_event::<NextLevel>()
        .add_event::<UpdateLevelStats>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct LevelStats {
    pub current_height: f32,
    pub blocks_thrown: usize,
    pub current_block_count: usize,
    pub blocks_dropped: usize,

    pub cars_hit: usize,
}

#[derive(Event, Debug, Clone)]
pub enum UpdateLevelStats {
    BlockThrown,
    BlockDestroyed,
    CarHit,
}

#[derive(Debug, Clone)]
pub struct LevelBase {
    pub base_type: BaseType,
    pub translation: Vec2,
    pub rotation: f32,
}

#[derive(Debug, Clone)]
pub enum LaunchPlatformKind {
    Static,
    Vertical,
    Free,
}

#[derive(Debug, Clone)]
pub struct LaunchPlatform {
    pub translation: Vec2,
    pub kind: LaunchPlatformKind,
}

const fn default_level_base() -> LevelBase {
    LevelBase {
        base_type: BaseType::T7,
        translation: Vec2::new(0.0, 10.0),
        rotation: 0.0,
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Level {
    pub level: usize,
    pub goal: LevelGoal,
    pub max_blocks: Option<usize>,
    pub time_limit: Option<Duration>,
    pub bases: &'static [LevelBase],
    pub enabled_effects: &'static [(EffectType, f32)],
    pub effect_likelihood: f32,
    pub intro_text: &'static str,
}

pub const DEFAULT_EFFECTS: [(EffectType, f32); 2] =
    [(EffectType::Glue, 1.0), (EffectType::Magnetic, 1.0)];

pub const DEFAULT_LEVEL: Level = Level {
    level: 0,
    goal: LevelGoal::ReachHeight(20.0),
    time_limit: Some(Duration::from_secs(60)),
    max_blocks: None,
    bases: &[LevelBase {
        base_type: BaseType::T9,
        ..default_level_base()
    }],
    enabled_effects: &DEFAULT_EFFECTS,
    effect_likelihood: 0.05,
    intro_text: "Welcome to the game!",
};

#[derive(Debug, Clone)]
pub enum LevelGoal {
    ReachHeight(f32),
    ReachBlockCount(usize),
}

#[derive(Event, Debug, Clone)]
pub struct NextLevel(pub Option<usize>);

#[derive(Component, Debug, Clone)]
pub struct LevelLifecycle;

pub static LEVELS: [Level; 6] = [
    Level {
        level: 0,
        intro_text: "Test Level",
        goal: LevelGoal::ReachHeight(15.0),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: None,
        bases: &[LevelBase {
            base_type: BaseType::T9,
            ..default_level_base()
        }],
        ..DEFAULT_LEVEL
    },
    Level {
        level: 1,
        intro_text: "Welcome to your first day at Big Bad Buildings, Inc. Your job is to operate the Tower Thrower 3000, a state-of-the-art machine that constructs buildings by throwing blocks.\
        For your first building, reach a target height of 200m.\
        ",
        goal: LevelGoal::ReachHeight(20.0),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: None,
        bases: &[LevelBase {
            base_type: BaseType::T9,
            ..default_level_base()
        }],
        ..DEFAULT_LEVEL
    },
    Level {
        level: 2,
        goal: LevelGoal::ReachBlockCount(10),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: Some(13),
        bases: &[LevelBase {
            base_type: BaseType::T4,
            ..default_level_base()
        }],
        ..DEFAULT_LEVEL
    },
    Level {
        level: 3,
        goal: LevelGoal::ReachHeight(20.0),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: Some(30),
        bases: &[LevelBase {
            base_type: BaseType::T4,
            ..default_level_base()
        }],
        ..DEFAULT_LEVEL
    },
    Level {
        level: 4,
        goal: LevelGoal::ReachBlockCount(20),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: Some(25),
        bases: &[LevelBase {
            base_type: BaseType::T4,
            ..default_level_base()
        }],
        ..DEFAULT_LEVEL
    },
    Level {
        level: 5,
        goal: LevelGoal::ReachHeight(35.0),
        time_limit: Some(Duration::from_secs(60)),
        max_blocks: Some(25),
        bases: &[
            LevelBase {
                base_type: BaseType::T2,
                translation: Vec2::new(80.0, 0.0),
                ..default_level_base()
            },
            LevelBase {
                base_type: BaseType::T2,
                translation: Vec2::new(-80.0, 0.0),
                ..default_level_base()
            },
        ],
        enabled_effects: &[(EffectType::Glue, 1.0)],
        ..DEFAULT_LEVEL
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

pub fn check_win_loose_condition(
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

fn update_level_stats_events(
    mut commands: Commands,
    mut level_stats: ResMut<LevelStats>,
    mut evr: EventReader<UpdateLevelStats>,
) {
    for event in evr.read() {
        match event {
            UpdateLevelStats::BlockThrown => {
                level_stats.blocks_thrown += 1;
            }
            UpdateLevelStats::BlockDestroyed => {
                level_stats.blocks_dropped += 1;
            }
            UpdateLevelStats::CarHit => {
                level_stats.cars_hit += 1;
            }
        }
    }
}
