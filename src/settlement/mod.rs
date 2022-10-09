use crate::{
    building::Building,
    game_state::{GameState, SettlementState},
    COIN_NAME,
};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

mod cap_resources;
mod close_by_keyboard;
mod close_event_handler;
mod settlement_ui;
mod trade_ui;
mod travel_ui;
mod ui;

const TRACK_PRODUCTION_TICKS: usize = 8;

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

pub struct CloseSettlementUIEvent;

#[derive(Debug, PartialEq, Eq)]
pub struct SelectedSettlement(Entity);

impl From<Entity> for SelectedSettlement {
    fn from(entity: Entity) -> SelectedSettlement {
        SelectedSettlement(entity)
    }
}

pub struct SettlementPlugin;

impl Plugin for SettlementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cap_resources::cap_resources)
            .add_system_set(
                SystemSet::on_update(GameState::Settlement(SettlementState::Overview))
                    .with_system(settlement_ui::settlement_ui)
                    .with_system(close_by_keyboard::close_by_keyboard)
                    .with_system(close_event_handler::close_event_handler),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Travel)
                    .with_system(travel_ui::travel_ui)
                    .with_system(close_by_keyboard::close_by_keyboard)
                    .with_system(close_event_handler::close_event_handler),
            )
            .add_system_set(
                // TODO: close by escape
                SystemSet::on_update(GameState::Settlement(SettlementState::Trade))
                    .with_system(trade_ui::trade_ui),
            );
    }
}
