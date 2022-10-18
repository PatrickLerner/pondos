use crate::game_state::{GameState, LoadingState, RunningState, SettlementState};
use bevy::prelude::*;
use iyes_loopless::{condition::ConditionSystemSet, prelude::*};

pub mod cap_resources;
mod settlement_ui;
mod trade_ui;
mod travel_ui;
mod ui;
mod visit_settlement_handler;

pub struct VisitSettlementEvent {
    pub settlement: Entity,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum SettlementLabel {
    CapResources,
}

pub struct SettlementPlugin;

fn build_set(game_state: GameState) -> ConditionSystemSet {
    ConditionSet::new()
        .run_in_bevy_state(game_state)
        .run_in_bevy_state(RunningState::Running)
        .run_in_bevy_state(LoadingState::Loaded)
        .with_system(crate::ui::close_by_keyboard)
        .with_system(crate::ui::close_event_handler)
}

impl Plugin for SettlementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(visit_settlement_handler::visit_settlement_handler)
            .add_system_set(
                build_set(GameState::Settlement(SettlementState::Overview))
                    .with_system(settlement_ui::settlement_ui)
                    .into(),
            )
            .add_system_set(
                build_set(GameState::Travel)
                    .with_system(travel_ui::travel_ui)
                    .into(),
            )
            .add_system_set(
                build_set(GameState::Settlement(SettlementState::Trade))
                    .with_system(trade_ui::trade_ui)
                    .into(),
            );
    }
}
