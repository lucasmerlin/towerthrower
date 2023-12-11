use bevy::prelude::*;

pub struct VisibilityTimerPlugin;

impl Plugin for VisibilityTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, visibility_timer);
    }
}

#[derive(Component, Debug)]
pub struct VisibilityTimer(pub Timer);

pub fn visibility_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut VisibilityTimer, &mut Visibility)>,
) {
    for (entity, mut timer, mut visible) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            *visible = Visibility::Visible;
            commands.entity(entity).remove::<VisibilityTimer>();
        }
    }
}
