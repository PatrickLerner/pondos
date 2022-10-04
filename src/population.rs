use crate::{
    game_time::{GameTime, GameTimeAdvancedEvent},
    settlement::Settlement,
    SeasonalAmount,
};
use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;
use std::collections::HashSet;

pub fn population_production(
    mut settlements: Query<&mut Settlement>,
    mut events: EventReader<GameTimeAdvancedEvent>,
    populations: Option<Res<Populations>>,
) {
    if let Some(populations) = populations {
        for event in events.iter() {
            for mut settlement in settlements.iter_mut() {
                settlement.production_tick(&event.time, &populations);
            }
        }
    }
}

#[derive(Deserialize, Component, Debug, Hash, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Population {
    pub name: String,
    pub production: Vec<Production>,
}

#[derive(Deserialize, Component, Debug, Hash, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Production {
    pub resource: String,
    pub amount: SeasonalAmount<u32>,
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "e7ac1c59-c2ac-4a77-9ace-532038a44758"]
pub struct Populations(HashSet<Population>);

impl Settlement {
    pub fn production_tick(&mut self, time: &GameTime, populations: &Populations) {
        for population in self.populations.clone() {
            let population = populations.0.iter().find(|i| i.name == population).unwrap();

            for production in population.production.iter() {
                let amount = production.amount.value(&time);

                let resource = if production.resource == "Gold" {
                    &mut self.gold
                } else {
                    self.resources
                        .entry(production.resource.clone())
                        .or_default()
                };

                *resource += amount;
            }
        }
    }
}
