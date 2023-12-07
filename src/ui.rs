use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
        app.add_systems(
            PreUpdate,
            (absorb_egui_inputs,)
                .after(bevy_egui::systems::process_input_system)
                .before(bevy_egui::EguiSet::BeginFrame),
        );
    }
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut button_color) in interaction_query.iter_mut() {
        *button_color = match interaction {
            Interaction::Hovered => Color::ORANGE_RED,
            Interaction::Pressed => Color::RED,
            _ => Color::GRAY,
        }
        .into();
    }
}

fn absorb_egui_inputs(mut mouse: ResMut<Input<MouseButton>>, mut contexts: EguiContexts) {
    if contexts.ctx_mut().is_pointer_over_area() {
        mouse.reset_all();
    }
}
