use crate::{game_time::GameTimeAdvanceEvent, player::PlayerTravelEvent, types::Player};
use bevy::prelude::*;

pub fn handle_travel(
    mut events: EventReader<PlayerTravelEvent>,
    mut player: Option<ResMut<Player>>,
    mut advance_time_events: EventWriter<GameTimeAdvanceEvent>,
) {
    for event in events.iter() {
        if let Some(player) = &mut player {
            if player.position != event.position {
                log::info!(
                    "Player traveled to {}:{}",
                    event.position.x,
                    event.position.y
                );
                player.update_position(event.position, Some(event.entity));
                advance_time_events.send(GameTimeAdvanceEvent);
            }
        }
    }
}
