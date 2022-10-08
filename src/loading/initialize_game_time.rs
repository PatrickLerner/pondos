use super::Settlements;
use crate::{
    game_time::{GameTime, GameTimeAdvanceEvent},
    population::Population,
    resources::Resource,
    Settings,
};
use bevy::prelude::*;

pub fn initialize_game_time(
    resources: Option<Res<Vec<Resource>>>,
    populations: Option<Res<Vec<Population>>>,
    settlement_handle: Option<Res<Handle<Settlements>>>,
    settings: Option<Res<Settings>>,
    game_time: Res<GameTime>,
    mut events: EventWriter<GameTimeAdvanceEvent>,
) {
    if settlement_handle.is_none()
        && resources.is_some()
        && populations.is_some()
        && settings.is_some()
        && !game_time.is_initialized()
    {
        events.send(GameTimeAdvanceEvent);
    }
}
