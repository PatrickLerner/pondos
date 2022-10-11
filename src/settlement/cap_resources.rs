use crate::{
    game_time::{GameTime, GameTimeAdvancedEvent},
    resources::Resource,
    settlement::Settlement,
    Settings,
};
use bevy::prelude::*;

pub fn cap_resources(
    mut settlements: Query<&mut Settlement>,
    mut events: EventReader<GameTimeAdvancedEvent>,
    resources: Option<Res<Vec<Resource>>>,
    settings: Option<Res<Settings>>,
) {
    if let Some(resources) = resources {
        if let Some(settings) = settings {
            for event in events.iter() {
                if event.time.is_initialized() {
                    log::debug!("capping max resources");
                }
                for mut settlement in settlements.iter_mut() {
                    settlement.resource_cap_tick(&event.time, &resources, &settings);
                }
            }
        }
    }
}

fn cap_resource(amount: &mut u32, multiplier: f32, max: u32, cap_percentage: f32) {
    let diff = f32::max(0., *amount as f32 - max as f32 * multiplier);

    *amount = ((*amount as f32 - (diff * cap_percentage)).floor() as u32).max(0);
}

impl Settlement {
    pub fn resource_cap_tick(
        &mut self,
        time: &GameTime,
        resources: &[Resource],
        settings: &Settings,
    ) {
        let multiplier = settings.max_multipliers.value(time);

        let max_silver = settings.max_silver.value(&self.populations).ceil() as u32;
        cap_resource(
            &mut self.silver,
            multiplier,
            max_silver,
            settings.cap_percentage,
        );
        let min_silver = settings.min_silver.value(&self.populations).ceil() as u32;
        if self.silver < min_silver {
            self.silver = min_silver;
        }

        for resource in resources.iter() {
            let max = resource.max.value(&self.populations).ceil() as u32;

            cap_resource(
                self.resources.entry(resource.name.clone()).or_default(),
                multiplier,
                max,
                settings.cap_percentage,
            );
        }
    }
}
