use crate::{
    game_time::{GameTime, GameTimeAdvanceEvent},
    population::Population,
    settlement::Resource,
    Settings,
};
use bevy::prelude::*;

pub fn initialize_game_time(
    resources: Option<Res<Vec<Resource>>>,
    populations: Option<Res<Vec<Population>>>,
    settings: Option<Res<Settings>>,
    game_time: Res<GameTime>,
    mut events: EventWriter<GameTimeAdvanceEvent>,
) {
    if resources.is_some() && populations.is_some() && settings.is_some() {
        if !game_time.is_initialized() {
            events.send(GameTimeAdvanceEvent);
        }
    }
}
