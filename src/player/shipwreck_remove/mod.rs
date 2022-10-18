use crate::{player::PlayerShipwreckEvent, types::Player};
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

#[cfg(test)]
mod tests;

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

        let lost_capacity = remove_index
            .iter()
            .fold(0, |acc, index| acc + player.convoy[*index].resource_space());

        log::info!(
            "{} ship(s) shipwrecked ({} capacity)",
            remove_index.len(),
            lost_capacity
        );

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

        let mut random = thread_rng();
        let mut options: Vec<String> = player
            .resources
            .iter()
            .filter_map(|(res, amount)| {
                if *amount > 0 {
                    Some(res.to_owned())
                } else {
                    None
                }
            })
            .collect();

        let resource_total = player.resource_space_used();
        let lost_resource_count = (resource_total as f32
            * (lost_capacity as f32 / (lost_capacity + player.resource_space_total()) as f32))
            .ceil() as u32;

        for _ in 0..lost_resource_count {
            options.shuffle(&mut random);

            if let Some(item) = options.first() {
                let res = player.resources.entry(item.clone()).or_default();
                *res -= 1;
                if *res == 0 {
                    options.remove(0);
                }
            }
        }
    }
}
