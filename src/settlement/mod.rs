use crate::{
    building::Building,
    game_state::{GameState, LoadingState, RunningState, SettlementState},
    COIN_NAME,
};
use bevy::prelude::*;
use iyes_loopless::{condition::ConditionSystemSet, prelude::*};
use serde::Deserialize;
use std::collections::HashMap;

pub mod cap_resources;
mod settlement_ui;
mod trade_ui;
mod travel_ui;
mod ui;

const TRACK_PRODUCTION_TICKS: usize = 8;

pub struct VisitSettlementEvent {
    pub settlement: Entity,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SettlementType {
    City,
    Outpost,
}

#[derive(Deserialize, Component, Debug)]
#[serde(deny_unknown_fields)]
pub struct Settlement {
    pub name: String,
    #[serde(rename = "type")]
    pub settlement_type: SettlementType,
    pub position: Position,
    #[serde(default)]
    pub silver: u32,
    #[serde(default)]
    pub resources: HashMap<String, u32>,
    pub populations: Vec<String>,
    #[serde(default, deserialize_with = "crate::building::building_deserialize")]
    pub buildings: Vec<Building>,
    #[serde(default)]
    production_last_ticks: Vec<HashMap<String, u32>>,
}

impl Settlement {
    pub fn track_production_tick(&mut self, production: HashMap<String, u32>) {
        self.production_last_ticks.push(production);
        if self.production_last_ticks.len() > TRACK_PRODUCTION_TICKS {
            self.production_last_ticks.remove(0);
        }
    }

    pub fn produced_items(&self) -> Vec<String> {
        let all_production =
            self.production_last_ticks
                .iter()
                .fold(HashMap::new(), |mut acc, items| {
                    for (res, amount) in items.iter() {
                        if res != COIN_NAME {
                            *acc.entry(res.clone()).or_default() += amount;
                        }
                    }

                    acc
                });

        let mut all_production: Vec<(String, u32)> = all_production.into_iter().collect();
        all_production.sort_unstable_by_key(|item| (-(item.1 as i32), item.0.clone()));
        all_production.into_iter().map(|item| item.0).collect()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum SettlementLabel {
    CapResources,
}

fn visit_settlement_handler(
    mut events: EventReader<VisitSettlementEvent>,
    mut game_state: ResMut<State<GameState>>,
    settlements: Query<&Settlement>,
) {
    for event in events.iter() {
        let settlement = settlements.get(event.settlement).unwrap();
        log::info!("Open settlement {}", settlement.name);
        game_state
            .push(GameState::Settlement(SettlementState::Overview))
            .unwrap();
    }
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
        app.add_system(visit_settlement_handler)
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
