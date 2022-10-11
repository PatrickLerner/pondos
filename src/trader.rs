use crate::{
    game_time::GameTimeAdvancedEvent,
    price_calculator::{AveragePrices, PriceCalculator},
    resources::Resource,
    settlement::Settlement,
};
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;

const MERCHANT: &str = "Merchant";
const BASE_ITEMS: usize = 2;
const ITEMS_PER_MERCHANT: usize = 3;

fn surplus_for_settlement(
    resources: &[Resource],
    average_prices: &AveragePrices,
    settlement: &Settlement,
) -> Vec<String> {
    let mut surplus: Vec<(String, u32)> = settlement.resources.clone().into_iter().collect();

    surplus.sort_unstable_by_key(|(resource_name, supply)| {
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

    surplus
        .into_iter()
        .map(|(resource_name, _)| resource_name)
        .collect()
}

fn item_count_for_settlement(settlement: &Settlement) -> usize {
    settlement
        .populations
        .iter()
        .filter(|p| *p == MERCHANT)
        .count()
        * ITEMS_PER_MERCHANT
        + BASE_ITEMS
}

pub fn trade_merchant(
    mut events: EventReader<GameTimeAdvancedEvent>,
    mut settlements: Query<(Entity, &mut Settlement)>,
    resources: Option<Res<Vec<Resource>>>,
    average_prices: Res<AveragePrices>,
) {
    if resources.is_none() {
        return;
    }
    let resources = resources.unwrap();

    for _ in events.iter() {
        let mut resource_pool: HashMap<String, u32> = HashMap::new();

        // push out surplus items
        for (_, mut settlement) in settlements.iter_mut() {
            let count = item_count_for_settlement(&settlement);

            for _ in 0..count {
                let surplus = surplus_for_settlement(&resources, &average_prices, &settlement);

                // get least demanded one
                if let Some(product) = surplus.last() {
                    *resource_pool.entry(product.clone()).or_default() += 1;
                    let amount = settlement.resources.entry(product.clone()).or_default();

                    if *amount > 0 {
                        *amount -= 1;
                    }
                }
            }
        }

        let mut picks = vec![];

        // take from common market
        for (entity, settlement) in settlements.iter() {
            let count = item_count_for_settlement(settlement);

            for _ in 0..count {
                picks.push(entity);
            }
        }

        picks.shuffle(&mut thread_rng());

        // take from common market
        for entity in picks.into_iter() {
            let (_, mut settlement) = settlements.get_mut(entity).unwrap();
            let surplus = surplus_for_settlement(&resources, &average_prices, &settlement);

            let product = surplus
                .clone()
                .into_iter()
                .find(|product| *resource_pool.entry(product.to_owned()).or_default() > 0);

            if let Some(product) = product {
                *resource_pool.entry(product.clone()).or_default() -= 1;
                *settlement.resources.entry(product.clone()).or_default() += 1;
            }
        }
    }
}
