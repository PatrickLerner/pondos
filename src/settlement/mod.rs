use std::collections::HashMap;

use crate::game_time::GameTime;

use super::GameState;
use bevy::prelude::*;
use serde::Deserialize;

mod cap_resources;
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
    pub resources: HashMap<Resource, u32>,
    pub populations: Vec<Population>,
}

fn cap_resource(amount: &mut u32, multiplier: f32, max: u32) {
    let diff = f32::max(0., *amount as f32 - max as f32 * multiplier);

    *amount = (*amount as f32 - (diff * 0.3)).floor() as u32;
}

impl Settlement {
    pub fn production_tick(&mut self, time: &GameTime) {
        let mut livestock_allocation = *self.resources.get(&Resource::Livestock).unwrap_or(&0);

        for population in self.populations.clone() {
            match population {
                Population::Hunter => {
                    *self.resources.entry(Resource::Meat).or_default() +=
                        if time.is_winter_season() { 2 } else { 7 };
                }
                Population::Farmer => {
                    if time.is_harvest_season() {
                        *self.resources.entry(Resource::Grain).or_default() += 10;
                    }
                    if time.is_summer_season() {
                        *self.resources.entry(Resource::Grain).or_default() += 3;
                    }
                    if time.is_growth_season() {
                        *self.resources.entry(Resource::Livestock).or_default() += 1;
                    }
                    if livestock_allocation >= 5 {
                        livestock_allocation -= 5;

                        *self.resources.entry(Resource::Meat).or_default() +=
                            if time.is_harvest_season() {
                                2
                            } else if time.is_growth_season() {
                                0
                            } else {
                                1
                            };
                        *self.resources.entry(Resource::Dairy).or_default() +=
                            if time.is_growth_season() { 0 } else { 2 };
                    }
                }
                Population::Fisher => {
                    *self.resources.entry(Resource::Fish).or_default() += if time.is_winter_season()
                    {
                        0
                    } else if time.is_summer_season() {
                        2
                    } else {
                        4
                    };
                }
                Population::Merchant => {
                    self.gold += if time.is_winter_season() { 10 } else { 35 };
                }
            }
        }
    }

    pub fn resource_cap_tick(&mut self, time: &GameTime) {
        let multiplier = if time.is_winter_season() {
            0.2
        } else if time.is_harvest_season() {
            1.5
        } else {
            1.0
        };

        let pops = self.populations.len() as u32;
        let farmers = self
            .populations
            .clone()
            .into_iter()
            .filter(|p| *p == Population::Farmer)
            .count() as u32;
        let merchants = self
            .populations
            .clone()
            .into_iter()
            .filter(|p| *p == Population::Merchant)
            .count() as u32;

        let max_gold = merchants * 90 + pops * 10;
        cap_resource(&mut self.gold, multiplier, max_gold);

        let max_grain = farmers * 3 + pops;
        let max_dairy = pops * 2;
        let max_meat = pops * 2;
        let max_fish = pops * 2;
        let max_livestock = farmers * 8 + pops;

        for (resource, max) in &[
            (Resource::Grain, max_grain),
            (Resource::Dairy, max_dairy),
            (Resource::Meat, max_meat),
            (Resource::Fish, max_fish),
            (Resource::Livestock, max_livestock),
        ] {
            cap_resource(
                self.resources.entry(*resource).or_default(),
                multiplier,
                *max,
            );
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize, Copy, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Population {
    Hunter,
    Farmer,
    Fisher,
    Merchant,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Resource {
    Grain,
    Dairy,
    Meat,
    Fish,
    Livestock,
}

impl Resource {
    pub fn base_price(&self) -> u32 {
        match self {
            Resource::Grain => 3,
            Resource::Dairy => 8,
            Resource::Meat => 15,
            Resource::Fish => 10,
            Resource::Livestock => 25,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SelectedSettlement(Entity);

impl From<Entity> for SelectedSettlement {
    fn from(entity: Entity) -> SelectedSettlement {
        SelectedSettlement(entity)
    }
}

pub struct CloseSettlementUIEvent;

pub fn close_by_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<CloseSettlementUIEvent>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        events.send(CloseSettlementUIEvent);
    }
}

pub struct SettlementPlugin;

impl Plugin for SettlementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cap_resources::cap_resources)
            .add_system_set(
                SystemSet::on_update(GameState::Settlement)
                    .with_system(settlement_ui::settlement_ui)
                    .with_system(close_by_keyboard)
                    .with_system(close_event_handler::close_event_handler),
            )
            .add_system_set(
                SystemSet::on_update(GameState::TravelToSettlement)
                    .with_system(travel_ui::travel_ui)
                    .with_system(close_by_keyboard)
                    .with_system(close_event_handler::close_event_handler),
            )
            .add_system_set(
                // TODO: close by escape
                SystemSet::on_update(GameState::TradeWithSettlement)
                    .with_system(trade_ui::trade_ui),
            );
    }
}
