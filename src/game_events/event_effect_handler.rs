use crate::{
    game_events::{GameEventEffect, TriggerEventEffect},
    player::Player,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn event_effect_handler(
    mut effects: EventReader<TriggerEventEffect>,
    mut player: Option<ResMut<Player>>,
) {
    if player.is_none() {
        return;
    }

    for event in effects.iter() {
        log::info!("trigger effect {:?}", event.effect);

        if let Some(player) = &mut player {
            match &event.effect {
                GameEventEffect::DamageAnyShip(damage) => {
                    if player.convoy.is_empty() {
                        return;
                    };

                    let mut random = thread_rng();
                    let index = random.gen_range(0..player.convoy.len());

                    let mut ship = &mut player.convoy[index];
                    ship.damage += damage.amount;
                }
                GameEventEffect::DamageAllShips(damage) => {
                    for ship in player.convoy.iter_mut() {
                        ship.damage += damage.amount;
                    }
                }
            }
        }
    }
}
