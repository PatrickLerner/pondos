use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, RichText},
    EguiContext,
};

use crate::{game_state::RunningState, ui::enabled_color};

pub struct GameTimeAdvancedEvent {
    pub time: GameTime,
}

pub struct GameTimeAdvanceEvent;

#[derive(Clone)]
pub struct GameTime {
    pub season: i8,
    pub year: i16,
}

impl GameTime {
    pub fn is_initialized(&self) -> bool {
        self.year > 1 || (self.year == 1 && self.season >= 2)
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            // we tick for a few years to generate resources on loading
            year: -1,
            season: 0,
        }
    }
}

impl GameTime {
    pub fn advance(&mut self) {
        self.season += 1;
        if self.season == 6 {
            self.season = 0;
            self.year += 1;
        }
    }

    pub fn is_growth_season(&self) -> bool {
        self.season == 1 || self.season == 2
    }

    pub fn is_summer_season(&self) -> bool {
        self.season == 3
    }

    pub fn is_harvest_season(&self) -> bool {
        self.season == 4
    }

    pub fn is_winter_season(&self) -> bool {
        self.season == 0 || self.season == 5
    }

    pub fn season_name(&self) -> &str {
        match self.season {
            0 => "Late winter",
            1 => "Early growth season",
            2 => "Late growth season",
            3 => "Summer",
            4 => "Harvest season",
            5 => "Early winter",
            _ => unreachable!("Unknown season"),
        }
    }
}

pub fn log_time(mut events: EventReader<GameTimeAdvancedEvent>) {
    for event in events.iter() {
        if event.time.is_initialized() {
            log::info!(
                "game time advanced - year {} season {}",
                event.time.year,
                event.time.season
            );
        }
    }
}

pub fn advance_time(
    mut events: EventReader<GameTimeAdvanceEvent>,
    mut game_time: ResMut<GameTime>,
    mut advanced_events: EventWriter<GameTimeAdvancedEvent>,
) {
    for _ in events.iter() {
        game_time.advance();
        advanced_events.send(GameTimeAdvancedEvent {
            time: game_time.clone(),
        })
    }
}

pub fn season_ui(
    mut egui_context: ResMut<EguiContext>,
    game_time: Res<GameTime>,
    mut events: EventWriter<GameTimeAdvanceEvent>,
    running_state: Res<State<RunningState>>,
) {
    egui::Window::new("Time")
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::RIGHT_TOP, (-14., 14.))
        .show(egui_context.ctx_mut(), |ui| {
            ui.set_width(220.0);
            ui.horizontal(|ui| {
                let enabled = running_state.current() == &RunningState::Running;
                if ui
                    .button(RichText::new("Wait").color(enabled_color(enabled)))
                    .clicked()
                    && enabled
                {
                    events.send(GameTimeAdvanceEvent)
                }

                ui.label(format!("Year {}", game_time.year));
                ui.label(game_time.season_name());
            });
        });
}

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(advance_time)
            .add_system(log_time)
            .add_system(season_ui);
    }
}
