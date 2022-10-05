use crate::{
    game_time::GameTime,
    population::Population,
    settlement::{Resource, Settlement},
    Settings,
};
use bevy::prelude::*;

use super::RequiresInitialization;

pub fn initialize_settlements(
    mut commands: Commands,
    mut uninitialized_settlements: Query<(Entity, &mut Settlement), With<RequiresInitialization>>,
    resources: Option<Res<Vec<Resource>>>,
    populations: Option<Res<Vec<Population>>>,
    settings: Option<Res<Settings>>,
) {
    if let Some(resources) = resources {
        if let Some(populations) = populations {
            if let Some(settings) = settings {
                if !uninitialized_settlements.is_empty() {
                    log::info!("initializing settlements");
                }

                for (entity, mut settlement) in uninitialized_settlements.iter_mut() {
                    let mut time = GameTime { year: 0, season: 0 };
                    for _ in 0..20 {
                        settlement.production_tick(&time, &populations);
                        settlement.resource_cap_tick(&time, &resources, &settings);
                        time.advance();
                    }

                    commands.entity(entity).remove::<RequiresInitialization>();
                }
            }
        }
    }
}
