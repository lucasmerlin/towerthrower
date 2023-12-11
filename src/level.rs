use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::base::BaseType;
use crate::block::{Aiming, Block, Falling};
use crate::effect::EffectType;
use crate::level_intro_dialog::DialogResource;
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

    pub timer: Option<Timer>,
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
    Free,
}

#[derive(Debug, Clone)]
pub struct LaunchPlatform {
    pub translation: Vec2,
    pub kind: LaunchPlatformKind,
}

const fn static_launch_platform() -> LaunchPlatform {
    LaunchPlatform {
        translation: Vec2::new(15.3, 10.8),
        kind: LaunchPlatformKind::Static,
    }
}

const fn free_launch_platform() -> LaunchPlatform {
    LaunchPlatform {
        translation: Vec2::new(15.3, 14.8),
        kind: LaunchPlatformKind::Free,
    }
}

const DEFAULT_BASE_HEIGHT: f32 = 11.0;

const fn default_level_base() -> LevelBase {
    LevelBase {
        base_type: BaseType::T7,
        translation: Vec2::new(0.0, DEFAULT_BASE_HEIGHT),
        rotation: 0.0,
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Level {
    pub level: usize,
    pub name: &'static str,
    pub goal: LevelGoal,
    pub max_blocks: Option<usize>,
    pub time_limit: Option<Duration>,
    pub bases: &'static [LevelBase],
    pub enabled_effects: &'static [(EffectType, f32)],
    pub effect_likelihood: f32,
    pub intro_text: Option<&'static str>,
    pub rain: Option<usize>,
    pub friction: f32,
    pub launch_platform: LaunchPlatform,
}

pub const DEFAULT_EFFECTS: [(EffectType, f32); 2] =
    [(EffectType::Glue, 1.0), (EffectType::Magnetic, 1.0)];

pub const NO_EFFECTS: [(EffectType, f32); 0] = [];

pub const DEFAULT_LEVEL: Level = Level {
    level: 0,
    name: "Unnamed",
    goal: LevelGoal::ReachHeight(20.0),
    time_limit: Some(Duration::from_secs(60)),
    max_blocks: None,
    bases: &[LevelBase {
        base_type: BaseType::T9,
        ..default_level_base()
    }],
    enabled_effects: &DEFAULT_EFFECTS,
    effect_likelihood: 0.05,
    intro_text: None,
    rain: None,
    friction: 0.5,
    launch_platform: static_launch_platform(),
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

pub static LEVELS: [Level; 10] = [
    Level {
        level: 0,
        name: "First day on the job",
        intro_text: Some("Welcome to your first day at Big Bad Buildings, Inc. Your job is to operate the Tower Thrower 3000, a state-of-the-art machine that constructs buildings by throwing blocks.
For your first building, reach a target height of 20m.

Controls:
Mouse: aim, 
Mouse wheel / Touch scroll: Adjust force
Right click: Rotate 90 degrees
Q / E: Finely adjust rotation
        "),
        goal: LevelGoal::ReachHeight(10.0),
        max_blocks: None,
        bases: &[LevelBase {
            base_type: BaseType::T9,
            ..default_level_base()
        }],
        enabled_effects: &NO_EFFECTS,
        ..DEFAULT_LEVEL
    },
    Level {
        level: 1,
        name: "Supply chain issues",
        intro_text: Some("For this building we only have a limited block supply. Be careful to not drop any! Stack 15 blocks to continue."),
        goal: LevelGoal::ReachBlockCount(15),
        max_blocks: Some(20),
        bases: &[LevelBase {
            base_type: BaseType::T7,
            ..default_level_base()
        }],
        enabled_effects: &NO_EFFECTS,
        ..DEFAULT_LEVEL
    },
    Level {
        level: 2,
        name: "Slip and Slide",
        intro_text: Some("Oh no, it's raining! Everything will be slippery"),
        goal: LevelGoal::ReachHeight(8.0),
        bases: &[LevelBase {
            base_type: BaseType::T7,
            ..default_level_base()
        }],
        rain: Some(10),
        friction: 0.2,
        enabled_effects: &NO_EFFECTS,
        launch_platform: LaunchPlatform {
          translation: Vec2::new(13.0, 10.5),
            kind: LaunchPlatformKind::Static,
        },
        ..DEFAULT_LEVEL
    },
    Level {
        level: 3,
        name: "Sticks like glue",
        intro_text: Some("We found some glue in the basement, some blocks will be sticky."),
        goal: LevelGoal::ReachHeight(12.0),
        max_blocks: Some(25),
        bases: &[LevelBase {
            base_type: BaseType::T2,
            translation: Vec2::new(-3.0, DEFAULT_BASE_HEIGHT + 1.0),
            ..default_level_base()
        }, LevelBase {
            translation: Vec2::new(4.0, DEFAULT_BASE_HEIGHT),
            base_type: BaseType::T3,
            ..default_level_base()
        }],
        enabled_effects: &[(EffectType::Glue, 1.0)],
        effect_likelihood: 0.1,
        ..DEFAULT_LEVEL
    },
    Level {
        level: 4,
        name: "I like to move it",
        intro_text: Some("Ooops, this one is tilted. We've upgraded your cannon with rocket boosters, so it can move freely now! Move with WASD."),
        goal: LevelGoal::ReachHeight(10.0),
        max_blocks: Some(20),
        bases: &[
            LevelBase {
                base_type: BaseType::T4,
                rotation: 0.1,
                ..default_level_base()
            },
        ],
        enabled_effects: &[(EffectType::Glue, 1.0)],
        launch_platform: free_launch_platform(),
        ..DEFAULT_LEVEL
    },
    Level {
        level: 5,
        name: "Head in the clouds",
        goal: LevelGoal::ReachBlockCount(15),
        max_blocks: Some(25),
        bases: &[
            LevelBase {
                base_type: BaseType::T4,
                ..default_level_base()
            },
        ],
        enabled_effects: &[(EffectType::Glue, 1.0)],
        launch_platform: free_launch_platform(),
        rain: Some(10),
        friction: 0.2,
        ..DEFAULT_LEVEL
    },
    Level {
        level: 6,
        name: "Attraction",
        intro_text: Some("We've ordered some magnets, these should hopefully help with building stability."),
        goal: LevelGoal::ReachHeight(30.0),
        bases: &[
            LevelBase {
                base_type: BaseType::T7,
                ..default_level_base()
            },
        ],
        enabled_effects: &[
            (EffectType::Glue, 1.0),
            (EffectType::Magnetic, 2.0),
        ],
        effect_likelihood: 0.1,
        launch_platform: free_launch_platform(),
        ..DEFAULT_LEVEL
    },

    Level {
        level: 7,
        name: "Double Trouble",
        intro_text: Some("We're going to build the next one on two existing buildings, try combining them so you have a wider fundament."),
        goal: LevelGoal::ReachHeight(20.0),
        bases: &[
            LevelBase {
                base_type: BaseType::T2,
                translation: Vec2::new(4.75, 10.0),
                ..default_level_base()
            },
            LevelBase {
                base_type: BaseType::T2,
                translation: Vec2::new(-4.75, 10.0),
                ..default_level_base()
            },
        ],
        launch_platform: free_launch_platform(),
        ..DEFAULT_LEVEL
    },
    Level {
        level: 8,
        name: "Block it like it's hot",
        intro_text: Some("Don't make any mistakes here"),
        goal: LevelGoal::ReachBlockCount(30),
        max_blocks: Some(33),
        bases: &[
            LevelBase {
                base_type: BaseType::T7,
                ..default_level_base()
            },
        ],
        launch_platform: free_launch_platform(),
        rain: Some(10),
        friction: 0.2,
        ..DEFAULT_LEVEL
    },
    Level {
        level: 9,
        name: "Hello, neighbors!",
        goal: LevelGoal::ReachHeight(22.0),
        bases: &[
            LevelBase {
                base_type: BaseType::T2,
                translation: Vec2::new(3.0, 10.0),
                rotation: 0.5,
                ..default_level_base()
            },
            LevelBase {
                base_type: BaseType::T2,
                translation: Vec2::new(-3.0, 10.0),
                rotation: -0.5,
                ..default_level_base()
            },
        ],
        launch_platform: free_launch_platform(),
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
    query: Query<(&Block, &Transform, &Velocity), (Without<Aiming>, Without<Falling>)>,
    level: Res<Level>,
    mut level_stats: ResMut<LevelStats>,
) {
    let mut max_height = 0.0;
    let mut block_count = 0;

    let base_height = level
        .bases
        .iter()
        .map(|base| base.translation.y)
        .max_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);

    for (block, transform, velocity) in query.iter() {
        if velocity.linvel.length() < 0.03 {
            block_count += 1;

            let corners = block.block_type.all_corners();

            let height = (corners
                .iter()
                .map(|corner| {
                    let pos = transform
                        .compute_matrix()
                        .transform_point(Vec3::from((*corner, 0.0)));
                    pos.y
                })
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0)
                - base_height)
                .max(0.0);

            if height > max_height {
                max_height = height;
            }
        }
    }

    level_stats.current_height = max_height;
    level_stats.current_block_count = block_count;
}

pub fn check_win_loose_condition(
    mut commands: Commands,
    mut level_stats: ResMut<LevelStats>,
    level: Res<Level>,
    mut state: ResMut<NextState<LevelState>>,
    time: Res<Time>,
) {
    match level.goal {
        LevelGoal::ReachHeight(height) => {
            // We add 0.05 because the ui is rounded a single decimal
            if level_stats.current_height + 0.05 >= height {
                state.set(LevelState::Won);
            }
        }
        LevelGoal::ReachBlockCount(count) => {
            if level_stats.current_block_count >= count {
                state.set(LevelState::Won);
            }
        }
    }

    if let Some(timer) = &mut level_stats.timer {
        timer.tick(time.delta());
        if timer.finished() {
            state.set(LevelState::Lost);
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
