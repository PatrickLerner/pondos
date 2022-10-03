use crate::{game_time::GameTimeAdvancedEvent, settlement::Settlement};
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
