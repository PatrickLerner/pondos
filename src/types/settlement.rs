use crate::{building::Building, COIN_NAME};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

const TRACK_PRODUCTION_TICKS: usize = 8;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SettlementType {
    City,
    Outpost,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub x: u32,
    pub y: u32,
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
