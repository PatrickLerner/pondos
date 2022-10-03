use super::Settlement;
use crate::game_time::GameTimeAdvancedEvent;
use bevy::prelude::*;

pub fn cap_resources(
    mut settlements: Query<&mut Settlement>,
    mut events: EventReader<GameTimeAdvancedEvent>,
) {
    for event in events.iter() {
        for mut settlement in settlements.iter_mut() {
            settlement.resource_cap_tick(&event.time);
        }
    }
}
