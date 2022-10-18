use crate::{player::PlayerShipwreckEvent, types::Player};
use bevy::prelude::*;

pub fn shipwreck_remove(
    player: Option<ResMut<Player>>,
    mut shipwreck_events: EventReader<PlayerShipwreckEvent>,
) {
    if shipwreck_events.is_empty() {
        return;
    }

    if let Some(mut player) = player {
        let remove_index: Vec<usize> = shipwreck_events
            .iter()
            .map(|event| event.ship_index)
            .collect();

        log::info!("{} ship(s) shipwrecked", remove_index.len());

        player.convoy = player
            .convoy
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(index, ship)| {
                if remove_index.contains(&index) {
                    None
                } else {
                    Some(ship)
                }
            })
            .collect();
    }
}
