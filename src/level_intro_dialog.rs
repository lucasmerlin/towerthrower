use crate::environment::fees::LevelFees;
use crate::level::{Level, LevelStats, NextLevel};
use crate::state::LevelState;
use bevy::prelude::*;
use bevy_egui::egui::{Color32, Frame};
use bevy_egui::{egui, EguiContext, EguiContexts};

pub struct LevelIntroDialogPlugin;

impl Plugin for LevelIntroDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup_level_intro_dialog)
            .add_systems(Update, update_level_intro_dialog)
            .init_resource::<DialogResource>();
    }
}

#[derive(Resource, Default)]
pub struct DialogResource {
    pub intro_visible: bool,
    pub results_visible: bool,
}

pub fn setup_level_intro_dialog(mut dialog: ResMut<DialogResource>) {
    dialog.intro_visible = true;
    dialog.results_visible = true;
}

pub fn update_level_intro_dialog(
    mut dialog: ResMut<DialogResource>,
    mut egui: EguiContexts,
    level: Res<Level>,
    level_state: Res<State<LevelState>>,
    stats: Res<LevelStats>,
    fees: Res<LevelFees>,
    mut next_level: EventWriter<NextLevel>,
    mut set_level_state: ResMut<NextState<LevelState>>,
) {
    if dialog.intro_visible {
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
                                dialog.intro_visible = false;
                            }
                        },
                    );
                });
        }
    }

    if dialog.results_visible {
        if *level_state == LevelState::Won || *level_state == LevelState::Lost {
            egui::Window::new("Level Complete")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(egui::Align2::LEFT_CENTER, egui::Vec2::new(16.0, 0.0))
                .constrain(true)
                .frame(
                    Frame::none()
                        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 220))
                        .inner_margin(8.0),
                )
                .show(egui.ctx_mut(), |ui| {
                    ui.set_width(200.0);

                    if *level_state == LevelState::Won {
                        ui.heading("Contract Completed!");
                        ui.label("Well Done!");
                    } else {
                        ui.heading("Contract Failed!");
                        ui.label("Better luck next time!");
                    }

                    ui.add_space(50.0);

                    ui.heading("Run Results");
                    ui.label(format!("Tower Height: {:.2}", stats.current_height));
                    ui.label(format!("Blocks Stacked: {}", stats.current_block_count));
                    ui.label(format!("Blocks Thrown: {}", stats.blocks_thrown));
                    ui.label(format!("Blocks Dropped: {}", stats.blocks_dropped));

                    ui.heading("Damage:");
                    ui.label(format!("Cleanup Fee: {:.2}$", fees.cleanup_fee));
                    ui.label(format!("Property Damage: {:.2}$", fees.property_damage));

                    ui.add_space(50.0);

                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            if ui.button("RETRY").clicked() {
                                next_level.send(NextLevel(Some(level.level)));
                            }
                            if *level_state == LevelState::Won {
                                if ui.button("NEXT").clicked() {
                                    next_level.send(NextLevel(None));
                                }
                            }
                            if ui.button("KEEP PLAYING").clicked() {
                                set_level_state.set(LevelState::KeepPlaying);
                            }
                        },
                    );
                });
        }
    }
}
