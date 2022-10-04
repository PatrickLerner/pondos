use std::collections::HashMap;

use crate::{CalculatedPopulationValue, GameState};
use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

mod cap_resources;
mod close_by_keyboard;
mod close_event_handler;
mod settlement_ui;
mod trade_ui;
mod travel_ui;

#[derive(Deserialize, Component, Debug)]
#[serde(deny_unknown_fields)]
pub struct Settlement {
    pub name: String,
    pub position: Position,
    #[serde(default)]
    pub gold: u32,
    #[serde(default)]
    pub resources: HashMap<String, u32>,
    pub populations: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "b8c204ad-f39e-4358-a88b-24d2c342140f"]
pub struct Resources(Vec<Resource>);

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Resource {
    name: String,
    base_price: u32,
    demand: CalculatedPopulationValue,
    max: CalculatedPopulationValue,
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
                SystemSet::on_update(GameState::Settlement)
                    .with_system(settlement_ui::settlement_ui)
                    .with_system(close_by_keyboard::close_by_keyboard)
                    .with_system(close_event_handler::close_event_handler),
            )
            .add_system_set(
                SystemSet::on_update(GameState::TravelToSettlement)
                    .with_system(travel_ui::travel_ui)
                    .with_system(close_by_keyboard::close_by_keyboard)
                    .with_system(close_event_handler::close_event_handler),
            )
            .add_system_set(
                // TODO: close by escape
                SystemSet::on_update(GameState::TradeWithSettlement)
                    .with_system(trade_ui::trade_ui),
            );
    }
}
