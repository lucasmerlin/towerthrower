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
        if let Some(intro_text) = level.intro_text {
            egui::Window::new("Level Intro")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .constrain(true)
                .frame(
                    Frame::none()
                        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 220))
                        .inner_margin(8.0),
                )
                .show(egui.ctx_mut(), |ui| {
                    ui.set_min_width(120.0);

                    ui.heading(format!("Level {} - {}", level.level + 1, level.name));

                    ui.label(intro_text);

                    ui.add_space(50.0);

                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            if ui.button("START").clicked() {
                                dialog.visible = false;
                            }
                        },
                    );
                });
        }
    }
}
