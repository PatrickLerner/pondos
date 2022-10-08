use bevy::prelude::*;
use std::collections::HashMap;

use crate::{game_time::GameTimeAdvancedEvent, resources::Resource, settlement::Settlement};

pub struct PriceCalculator {
    pub base_price: u32,
    pub demand: u32,
    pub supply: u32,
}

const MIN_SHORTAGE_MOD: f32 = 0.8;
const MAX_SHORTAGE_MOD: f32 = 2.0;

impl PriceCalculator {
    fn shortage_mod(&self) -> f32 {
        let res = (self.demand as f32 - self.supply as f32) / self.demand as f32;

        f32::clamp(1. + res, MIN_SHORTAGE_MOD, MAX_SHORTAGE_MOD)
    }

    /// price for which player can buy
    pub fn buy_price(&self) -> u32 {
        (self.base_price as f32 * self.shortage_mod()).ceil() as u32
    }

    /// price for which player can sell
    pub fn sell_price(&self) -> u32 {
        let price = Self {
            base_price: self.base_price,
            demand: self.demand,
            supply: self.supply + 1,
        };

        // we would buy for price we could sell it at if we had one more
        price.buy_price()
    }
}

#[derive(Default)]
pub struct AveragePrices {
    pub prices: HashMap<String, f32>,
}

pub fn average_prices(
    settlements: Query<&Settlement>,
    mut average_prices: ResMut<AveragePrices>,
    resources: Option<Res<Vec<Resource>>>,
    events: EventReader<GameTimeAdvancedEvent>,
) {
    if events.is_empty() || resources.is_none() {
        return;
    }

    let resources = resources.unwrap();

    let settlement_count = settlements.iter().len() as f32;

    for resource in resources.iter() {
        let sum = settlements.iter().fold(0.0, |acc, settlement| {
            let demand = resource.demand.value(&settlement.populations).ceil() as u32;

            let prices = PriceCalculator {
                base_price: resource.base_price,
                demand,
                supply: *settlement.resources.get(&resource.name).unwrap_or(&0),
            };

            acc + prices.sell_price() as f32
        });

        *average_prices
            .prices
            .entry(resource.name.clone())
            .or_default() = sum / settlement_count;
    }
}
