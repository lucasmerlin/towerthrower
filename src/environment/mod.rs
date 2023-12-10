use bevy::prelude::*;

mod beam;
mod car;
pub mod city;
mod debris_cleaner;
pub mod fees;
pub mod rain;
mod tow_truck;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debris_cleaner::DebrisCleanerPlugin,
            car::CarPlugin,
            tow_truck::TowTruckPlugin,
            beam::BeamPlugin,
            fees::FeesPlugin,
            city::CityPlugin,
            rain::RainPlugin,
        ));
    }
}
