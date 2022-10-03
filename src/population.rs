use crate::{
    game_time::{GameTime, GameTimeAdvancedEvent},
    settlement::{Population, Resource, Settlement},
};
use bevy::prelude::*;

pub fn population_production(
    mut settlements: Query<&mut Settlement>,
    mut events: EventReader<GameTimeAdvancedEvent>,
) {
    for event in events.iter() {
        for mut settlement in settlements.iter_mut() {
            settlement.production_tick(&event.time);
        }
    }
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
}
