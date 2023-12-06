use bevy::prelude::*;
use std::time::Duration;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelStats>()
            .insert_resource(LEVELS[0].clone());
    }
}

#[derive(Resource, Debug, Default)]
pub struct LevelStats {
    pub blocks_thrown: usize,
    pub blocks_caught: usize,
    pub blocks_missed: usize,

    pub deaths: usize,
}

#[derive(Resource, Debug, Clone)]
pub struct Level {
    pub level: usize,
    pub goal: LevelGoal,
    pub max_deaths: Option<usize>,
    pub max_blocks: Option<usize>,
    pub time_limit: Option<Duration>,
    pub base_width: f32,
}

#[derive(Debug, Clone)]
pub enum LevelGoal {
    ReachHeight(f32),
    ReachBlockCount(usize),
}

pub static LEVELS: [Level; 1] = [Level {
    level: 1,
    goal: LevelGoal::ReachHeight(200.0),
    time_limit: Some(Duration::from_secs(60)),
    max_blocks: Some(10),
    max_deaths: None,
    base_width: 160.0,
}];
