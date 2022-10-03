use crate::{
    game_time::GameTime,
    population::Populations,
    settlement::{Resources, Settlement},
};
use bevy::prelude::*;

use super::RequiresInitialization;

pub fn initialize_settlements(
    mut commands: Commands,
    mut uninitialized_settlements: Query<(Entity, &mut Settlement), With<RequiresInitialization>>,
    resources: Option<Res<Resources>>,
    populations: Option<Res<Populations>>,
) {
    if let Some(resources) = resources {
        if let Some(populations) = populations {
            if !uninitialized_settlements.is_empty() {
                log::info!("initializing settlements");
            }

            for (entity, mut settlement) in uninitialized_settlements.iter_mut() {
                let mut time = GameTime { year: 0, season: 0 };
                for _ in 0..20 {
                    settlement.production_tick(&time, &populations);
                    settlement.resource_cap_tick(&time, &resources);
                    time.advance();
                }

                commands.entity(entity).remove::<RequiresInitialization>();
            }
        }
    }
}
