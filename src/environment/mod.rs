use bevy::prelude::*;

mod beam;
mod car;
mod debris_cleaner;
mod tow_truck;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debris_cleaner::DebrisCleanerPlugin,
            car::CarPlugin,
            tow_truck::TowTruckPlugin,
            beam::BeamPlugin,
        ));
    }
}
