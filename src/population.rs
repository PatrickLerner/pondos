use crate::{
    game_time::{GameTime, GameTimeAdvancedEvent},
    settlement::Settlement,
    types::SeasonalAmount,
};
use bevy::prelude::*;
use serde::Deserialize;

pub fn population_production(
    mut settlements: Query<&mut Settlement>,
    mut events: EventReader<GameTimeAdvancedEvent>,
    populations: Option<Res<Vec<Population>>>,
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

impl Settlement {
    pub fn production_tick(&mut self, time: &GameTime, populations: &[Population]) {
        for population in self.populations.clone() {
            let population = populations.iter().find(|i| i.name == population).unwrap();

            for production in population.production.iter() {
                let amount = production.amount.value(time);

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
