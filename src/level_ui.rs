use bevy::prelude::*;
use bevy_egui::egui::{Color32, RichText};
use bevy_egui::{egui, EguiContexts};

use crate::level::{Level, LevelStats, NextLevel};
use crate::state::LevelState;

pub struct LevelUiPlugin;

impl Plugin for LevelUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level_ui)
            .add_systems(Update, egui_level_ui);
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
