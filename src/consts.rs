use bevy_rapier2d::prelude::Group;

// Building collision groups
pub const BLOCK_COLLISION_GROUP: Group = Group::GROUP_1;
pub const BASE_COLLISION_GROUP: Group = Group::GROUP_6;

// Foreground collision groups
pub const FLOOR_COLLISION_GROUP: Group = Group::GROUP_2;
pub const DEBRIS_COLLISION_GROUP: Group = Group::GROUP_3;
pub const RAIN_COLLISION_GROUP: Group = Group::GROUP_4;
pub const VEHICLE_COLLISION_GROUP: Group = Group::GROUP_5;

// Combinations
pub fn foreground_collision_groups() -> Group {
    FLOOR_COLLISION_GROUP | DEBRIS_COLLISION_GROUP | RAIN_COLLISION_GROUP | VEHICLE_COLLISION_GROUP
}
pub fn building_collision_groups() -> Group {
    BLOCK_COLLISION_GROUP | BASE_COLLISION_GROUP
}
