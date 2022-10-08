use crate::{game_state::GameState, game_time::GameTimeAdvancedEvent, player::TransportType};
use bevy::prelude::*;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

mod cap_resources;
mod close_by_keyboard;
mod close_event_handler;
mod settlement_ui;
mod shipyard_ui;
mod trade_ui;
mod travel_ui;

const WINDOW_PADDING_X: f32 = 40.;
const WINDOW_PADDING_Y: f32 = 80.;
const MAX_WIDTH: f32 = 800.;
const MAX_HEIGHT: f32 = 640.;

#[derive(Deserialize, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    pub entity: Entity,
}

impl From<BuildingType> for Building {
    fn from(building_type: BuildingType) -> Self {
        Self {
            building_type,
            entity: Entity::from_raw(0),
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Shipyard {
    construction: Option<TransportType>,
    construction_time: u32,
}

#[derive(Deserialize, Debug)]
pub enum BuildingType {
    Shipyard,
}

fn building_deserialize<'de, D>(deserializer: D) -> Result<Vec<Building>, D::Error>
where
    D: Deserializer<'de>,
{
    let building_types = Vec::<BuildingType>::deserialize(deserializer)?;

    Ok(building_types
        .into_iter()
        .map(|building_type| building_type.into())
        .collect())
}

fn shipyard_construction(
    mut shipyards: Query<&mut Shipyard>,
    mut events: EventReader<GameTimeAdvancedEvent>,
) {
    for _ in events.iter() {
        for mut shipyard in shipyards.iter_mut() {
            if shipyard.construction_time > 0 {
                shipyard.construction_time -= 1;
            }
        }
    }
}

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
    #[serde(default, deserialize_with = "building_deserialize")]
    pub buildings: Vec<Building>,
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

#[derive(Debug, PartialEq, Eq)]
pub struct SelectedBuilding(Entity);

impl From<Entity> for SelectedSettlement {
    fn from(entity: Entity) -> SelectedSettlement {
        SelectedSettlement(entity)
    }
}

pub struct SettlementPlugin;

impl Plugin for SettlementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cap_resources::cap_resources)
            .add_system(shipyard_construction)
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
            )
            .add_system_set(
                // TODO: close by escape
                SystemSet::on_update(GameState::Shipyard).with_system(shipyard_ui::shipyard_ui),
            );
    }
}
