use crate::{
    game_time::{GameTime, GameTimeAdvancedEvent},
    settlement::{Population, Settlement},
};
use bevy::prelude::*;

use super::Resources;

pub fn cap_resources(
    mut settlements: Query<&mut Settlement>,
    mut events: EventReader<GameTimeAdvancedEvent>,
    resources: Option<Res<Resources>>,
) {
    if let Some(resources) = resources {
        for event in events.iter() {
            for mut settlement in settlements.iter_mut() {
                settlement.resource_cap_tick(&event.time, &resources);
            }
        }
    }
}

fn cap_resource(amount: &mut u32, multiplier: f32, max: u32) {
    let diff = f32::max(0., *amount as f32 - max as f32 * multiplier);

    *amount = (*amount as f32 - (diff * 0.3)).floor() as u32;
}

impl Settlement {
    pub fn resource_cap_tick(&mut self, time: &GameTime, resources: &Resources) {
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

        for resource in resources.0.iter() {
            let max = farmers * resource.max_farmer_mod + pops * resource.max_pop_mod;

            cap_resource(
                self.resources.entry(resource.name.clone()).or_default(),
                multiplier,
                max,
            );
        }
    }
}
