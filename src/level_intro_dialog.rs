use crate::level::Level;
use crate::state::LevelState;
use bevy::prelude::*;
use bevy_egui::egui::{Color32, Frame};
use bevy_egui::{egui, EguiContext, EguiContexts};

pub struct LevelIntroDialogPlugin;

impl Plugin for LevelIntroDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup_level_intro_dialog)
            .add_systems(Update, update_level_intro_dialog)
            .init_resource::<LevelIntroDialog>();
    }
}

#[derive(Resource, Default)]
pub struct LevelIntroDialog {
    pub visible: bool,
}

pub fn setup_level_intro_dialog(mut dialog: ResMut<LevelIntroDialog>) {
    dialog.visible = true;
}

pub fn update_level_intro_dialog(
    mut dialog: ResMut<LevelIntroDialog>,
    mut egui: EguiContexts,
    level: Res<Level>,
) {
    if dialog.visible {
        egui::Window::new("Level Intro")
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .frame(
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 250))
                    .inner_margin(8.0),
            )
            .show(egui.ctx_mut(), |ui| {
                ui.set_min_size(egui::Vec2::new(400.0, 300.0));

                ui.heading("Level Intro");

                ui.label(level.intro_text);

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("START").clicked() {
                        dialog.visible = false;
                    }
                });
            });
    }
}