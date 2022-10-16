use crate::{
    game_events::{GameEvent, GameEventsState, TriggerEvent},
    game_state::RunningState,
};
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashMap;

pub fn event_trigger_handler(
    mut triggers: EventReader<TriggerEvent>,
    events: Option<Res<HashMap<String, GameEvent>>>,
    mut state: ResMut<GameEventsState>,
    mut running_state: ResMut<State<RunningState>>,
) {
    if events.is_none() {
        return;
    };
    let events = events.unwrap();
    let mut added_events = false;

    for trigger in triggers.iter() {
        let mut random = thread_rng();
        let mut events: Vec<&GameEvent> = events
            .iter()
            .filter_map(|(_, event)| {
                if let Some(event_trigger) = &event.trigger {
                    if event_trigger.once && state.seen_events.contains(&event.id) {
                        return None;
                    }

                    if event_trigger.event != trigger.event || event_trigger.scope != trigger.scope
                    {
                        return None;
                    }

                    if let Some(chance) = event_trigger.chance {
                        if random.gen_range(0.0..1.0) > chance {
                            return None;
                        }
                    }
                }

                Some(event)
            })
            .collect();

        events.shuffle(&mut thread_rng());

        if let Some(event) = events.first() {
            log::info!("trigger game event {}", event.id);
            state.current_events.insert(event.id.to_owned());
            state.seen_events.insert(event.id.to_owned());
            added_events = true;
        }
    }

    if added_events {
        running_state.overwrite_set(RunningState::Paused).unwrap();
    }
}
