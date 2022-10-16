use crate::{game_events::TriggerEventEffect, player::Player};
use bevy::prelude::*;

pub fn event_effect_handler(
    mut effects: EventReader<TriggerEventEffect>,
    mut player: Option<ResMut<Player>>,
) {
    for effect in effects.iter() {
        log::info!("trigger effect {:?}", effect);
    }
}
