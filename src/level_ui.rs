use bevy::prelude::*;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Color32, Frame, ImageSource, Layout, RichText};
use bevy_egui::{egui, EguiContexts};

use crate::block::Aiming;
use crate::environment::fees::LevelFees;
use crate::level::{Level, LevelGoal, LevelStats, NextLevel};
use crate::state::LevelState;
use crate::throw::ThrowQueue;

pub struct LevelUiPlugin;

impl Plugin for LevelUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level_ui)
            .add_systems(Update, (egui_level_ui, target_ui));
    }
}

#[derive(Component, Debug)]
struct StatsText;

#[derive(Component, Debug)]
struct RetryButton;

#[derive(Component, Debug)]
struct NextLevelButton;

pub fn setup_level_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                StatsText,
                TextBundle::from_section("Stats", TextStyle::default()).with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                }),
            ));
            parent
                .spawn((RetryButton, ButtonBundle::default()))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Retry", TextStyle::default()));
                });
        });
}

pub fn egui_level_ui(
    mut contexts: EguiContexts,
    mut next_level: EventWriter<NextLevel>,
    stats: Res<LevelStats>,
    current_level: Res<Level>,
    level_state: Res<State<LevelState>>,
    fees: Res<LevelFees>,
) {
    egui::Window::new("Level UI")
        .title_bar(false)
        .movable(false)
        .resizable(false)
        //.frame(Frame::none())
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Level UI");

            ui.heading("Stats:");
            ui.label(format!("Tower Height: {:.2}", stats.current_height));
            ui.label(format!("Blocks Stacked: {}", stats.current_block_count));
            ui.label(format!("Blocks Thrown: {}", stats.blocks_thrown));
            ui.label(format!("Blocks Dropped: {}", stats.blocks_dropped));

            ui.heading("Damage:");
            ui.label(format!("Cleanup Fee: {:.2}$", fees.cleanup_fee));
            ui.label(format!("Property Damage: {:.2}$", fees.property_damage));

            ui.heading("Winning Condition:");
            match current_level.goal {
                crate::level::LevelGoal::ReachHeight(height) => {
                    ui.label(format!("Reach Height: {:.2}", height));
                }
                crate::level::LevelGoal::ReachBlockCount(count) => {
                    ui.label(format!("Reach Block Count: {}", count));
                }
            }

            ui.heading("Loosing Condition:");
            if let Some(max_blocks) = current_level.max_blocks {
                ui.label(format!("Max Blocks: {}", max_blocks));
            }

            if ui.button("Next Level").clicked() {
                next_level.send(NextLevel(None));
            }
            if current_level.level > 0 {
                if ui.button("Previous Level").clicked() {
                    next_level.send(NextLevel(Some(current_level.level - 1)));
                }
            }
            if ui.button("Retry").clicked() {
                next_level.send(NextLevel(Some(current_level.level)));
            }

            if *level_state == LevelState::Won {
                ui.label(RichText::new("You Won").size(50.0).color(Color32::GREEN));
            }

            if *level_state == LevelState::Lost {
                ui.label(RichText::new("You Lost").size(50.0).color(Color32::RED));
            }
        });
}

pub fn target_ui(
    level: Res<Level>,
    level_stats: Res<LevelStats>,
    mut egui: EguiContexts,
    assets: Res<AssetServer>,
    mut is_initialized: Local<bool>,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut has_aiming_block: Query<(), With<Aiming>>,
    queue: Res<ThrowQueue>,
) {
    if !*is_initialized {
        *is_initialized = true;
        *rendered_texture_id = egui.add_image(assets.load("blocks/T/2.png"));
    }

    egui::Window::new("Target UI")
        .title_bar(false)
        .movable(false)
        .resizable(false)
        .frame(Frame::none())
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-16.0, 8.0))
        .show(egui.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                match level.goal {
                    LevelGoal::ReachHeight(height) => {
                        ui.horizontal(|ui| {
                            ui.set_min_height(45.0);
                            ui.label(
                                RichText::new(format!("/ {:.1}m", height))
                                    .size(30.0)
                                    .color(Color32::DARK_GRAY),
                            );
                            ui.label(
                                RichText::new(format!("{:.1}m", level_stats.current_height))
                                    .size(40.0)
                                    .color(Color32::BLACK),
                            );
                        });
                    }
                    LevelGoal::ReachBlockCount(count) => {
                        ui.horizontal(|ui| {
                            ui.set_min_height(45.0);
                            ui.label(
                                RichText::new(format!("/ {}", count))
                                    .size(30.0)
                                    .color(Color32::DARK_GRAY),
                            );
                            ui.label(
                                RichText::new(format!("{}", level_stats.current_block_count))
                                    .size(40.0)
                                    .color(Color32::BLACK),
                            );
                        });
                    }
                }

                if let Some(max_blocks) = level.max_blocks {
                    let add_one_for_aiming_block = if has_aiming_block.get_single().is_ok() {
                        1
                    } else {
                        0
                    };

                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "{}",
                                queue.queue.len() + add_one_for_aiming_block
                            ))
                            .size(40.0)
                            .color(Color32::BLACK),
                        );
                        ui.image(ImageSource::Texture(SizedTexture::new(
                            *rendered_texture_id,
                            egui::Vec2::new(60.0, 40.0),
                        )));
                    });
                }
            });
        });
}
