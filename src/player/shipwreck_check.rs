use crate::{player::PlayerShipwreckEvent, types::Player};
use bevy::prelude::*;

pub fn shipwreck_check(
    player: Option<Res<Player>>,
    mut shipwreck_events: EventWriter<PlayerShipwreckEvent>,
) {
    if let Some(player) = player {
        for (ship_index, ship) in player.convoy.iter().enumerate() {
            if ship.health() == 0. {
                shipwreck_events.send(PlayerShipwreckEvent { ship_index });
            }
        }
    }
}
