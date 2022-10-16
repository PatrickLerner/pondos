use crate::game_events::{AddEventToCurrentEvent, GameEvent, GameEventsState, TriggerEventEffect};
use crate::game_state::RunningState;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn add_event_to_current_event(
    mut event_triggers: EventReader<AddEventToCurrentEvent>,
    events: Option<Res<HashMap<String, GameEvent>>>,
    mut state: ResMut<GameEventsState>,
    mut effects: EventWriter<TriggerEventEffect>,
    mut running_state: ResMut<State<RunningState>>,
) {
    for event_trigger in event_triggers.iter() {
        if let Some(events) = &events {
            let event = events.get(&event_trigger.id).unwrap();
            log::info!("trigger game event {}", event_trigger.id);

            state.current_events.insert(event.id.to_owned());
            state.seen_events.insert(event.id.to_owned());
            for effect in event.effects.clone() {
                effects.send(TriggerEventEffect { effect });
            }
        }
    }

    if state.current_events.is_empty() && *running_state.current() != RunningState::Running {
        running_state.overwrite_set(RunningState::Running).unwrap();
    }

    if !state.current_events.is_empty() && *running_state.current() != RunningState::Paused {
        running_state.overwrite_set(RunningState::Paused).unwrap();
    }
}
