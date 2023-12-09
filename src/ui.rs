use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts, EguiSettings};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_ui_scale_factor));
        app.add_systems(
            PreUpdate,
            (absorb_egui_inputs,)
                .after(bevy_egui::systems::process_input_system)
                .before(bevy_egui::EguiSet::BeginFrame),
        );
        app.add_systems(Startup, egui_setup);
    }
}

fn absorb_egui_inputs(mut mouse: ResMut<Input<MouseButton>>, mut contexts: EguiContexts) {
    if contexts.ctx_mut().is_pointer_over_area() {
        mouse.reset_all();
    }
}

fn update_ui_scale_factor(
    mut egui_settings: ResMut<EguiSettings>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = windows.get_single() {
        egui_settings.scale_factor = 1.25;
    }
}

fn egui_setup(mut egui: EguiContexts) {
    let ctx = egui.ctx_mut();

    ctx.style_mut(|style| {
        style.spacing.button_padding = egui::Vec2::new(16.0, 8.0);

        // style
        //     .text_styles
        //     .get_mut(&egui::TextStyle::Button)
        //     .unwrap()
        //     .size = 24.0;
    });
}
