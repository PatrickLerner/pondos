use crate::{
    game_time::{GameTime, GameTimeAdvancedEvent},
    settlement::Settlement,
    Settings,
};
use bevy::prelude::*;

use super::Resources;

pub fn cap_resources(
    mut settlements: Query<&mut Settlement>,
    mut events: EventReader<GameTimeAdvancedEvent>,
    resources: Option<Res<Resources>>,
    settings: Option<Res<Settings>>,
) {
    if let Some(resources) = resources {
        if let Some(settings) = settings {
            for event in events.iter() {
                for mut settlement in settlements.iter_mut() {
                    settlement.resource_cap_tick(&event.time, &resources, &settings);
                }
            }
        }
    }
}

fn cap_resource(amount: &mut u32, multiplier: f32, max: u32) {
    let diff = f32::max(0., *amount as f32 - max as f32 * multiplier);

    *amount = (*amount as f32 - (diff * 0.3)).floor() as u32;
}

impl Settlement {
    pub fn resource_cap_tick(
        &mut self,
        time: &GameTime,
        resources: &Resources,
        settings: &Settings,
    ) {
        let multiplier = settings.max_multipliers.value(time);

        let max_gold = settings.max_gold.value(&self.populations).ceil() as u32;
        cap_resource(&mut self.gold, multiplier, max_gold);

        for resource in resources.0.iter() {
            let max = resource.max.value(&self.populations).ceil() as u32;

            cap_resource(
                self.resources.entry(resource.name.clone()).or_default(),
                multiplier,
                max,
            );
        }
    }
}
