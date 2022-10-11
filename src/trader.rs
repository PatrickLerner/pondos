use std::collections::HashMap;

use crate::{
    game_time::GameTimeAdvancedEvent,
    price_calculator::{AveragePrices, PriceCalculator},
    resources::Resource,
    settlement::Settlement,
};
use bevy::prelude::*;

const MERCHANT: &str = "Merchant";

pub fn trade_merchant(
    mut events: EventReader<GameTimeAdvancedEvent>,
    mut settlements: Query<&mut Settlement>,
    resources: Option<Res<Vec<Resource>>>,
    average_prices: Res<AveragePrices>,
) {
    if resources.is_none() {
        return;
    }
    let resources = resources.unwrap();

    let item_per_merchant = 5;

    for _ in events.iter() {
        let mut resource_pool: HashMap<String, u32> = HashMap::new();

        // push out surplus items
        for mut settlement in settlements.iter_mut() {
            let count = settlement
                .populations
                .iter()
                .filter(|p| *p == MERCHANT)
                .count()
                + 1;

            for _ in 0..(count * item_per_merchant) {
                let mut demand: Vec<(String, u32)> =
                    settlement.resources.clone().into_iter().collect();

                demand.sort_unstable_by_key(|(resource_name, supply)| {
                    let resource = resources.iter().find(|r| r.name == *resource_name).unwrap();
                    let demand = resource.demand.value(&settlement.populations).ceil() as u32;
                    let prices = PriceCalculator {
                        base_price: resource.base_price,
                        demand,
                        supply: *supply,
                    };

                    let price = prices.sell_price();

                    if let Some(average_price) = average_prices.prices.get(resource_name) {
                        (average_price / price as f32 * 1000.) as u32
                    } else {
                        0
                    }
                });

                demand.reverse();

                if demand.len() > 0 {
                    let product = demand[0].0.clone();
                    *resource_pool.entry(product.clone()).or_default() += 1;
                    let amount = settlement.resources.entry(product.clone()).or_default();

                    if *amount > 0 {
                        *amount -= 1;
                    }
                }
            }
        }

        // take from common market
        for mut settlement in settlements.iter_mut() {
            let count = settlement
                .populations
                .iter()
                .filter(|p| *p == MERCHANT)
                .count()
                + 1;

            let mut demand: Vec<(String, u32)> = settlement.resources.clone().into_iter().collect();

            demand.sort_unstable_by_key(|(resource_name, supply)| {
                let resource = resources.iter().find(|r| r.name == *resource_name).unwrap();
                let demand = resource.demand.value(&settlement.populations).ceil() as u32;
                let prices = PriceCalculator {
                    base_price: resource.base_price,
                    demand,
                    supply: *supply,
                };

                let price = prices.sell_price();

                if let Some(average_price) = average_prices.prices.get(resource_name) {
                    (average_price / price as f32 * 1000.) as u32
                } else {
                    0
                }
            });

            let demand_goods: Vec<String> = demand.into_iter().map(|(res, _)| res).collect();

            for _ in 0..(count * item_per_merchant) {
                let good = demand_goods
                    .clone()
                    .into_iter()
                    .find(|product| *resource_pool.entry(product.to_owned()).or_default() > 0);

                if let Some(good) = good {
                    *resource_pool.entry(good.clone()).or_default() -= 1;
                    *settlement.resources.entry(good.clone()).or_default() += 1;
                }
            }
        }
    }
}
