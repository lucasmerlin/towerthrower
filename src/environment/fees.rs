use crate::level::LevelStats;
use crate::state::LevelState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct FeesPlugin;

impl Plugin for FeesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelFees>()
            .add_systems(OnEnter(LevelState::Playing), reset_level_fees)
            .add_systems(Update, update_fees.run_if(in_state(LevelState::Playing)));
    }
}

#[derive(Resource, Debug, Default)]
pub struct LevelFees {
    pub cleanup_fee: f32,
    pub property_damage: f32,
}

pub fn update_fees(mut level_fees: ResMut<LevelFees>, stats: Res<LevelStats>) {
    level_fees.cleanup_fee = stats.blocks_dropped as f32 * 100.0;
    level_fees.property_damage = stats.cars_hit as f32 * 10000.0;
}

pub fn reset_level_fees(mut level_fees: ResMut<LevelFees>) {
    *level_fees = LevelFees::default();
}
