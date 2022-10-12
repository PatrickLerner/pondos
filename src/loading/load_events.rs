use super::GameEvents;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn load_events(
    mut commands: Commands,
    events_handle: Option<Res<Handle<GameEvents>>>,
    mut events: ResMut<Assets<GameEvents>>,
) {
    if let Some(events_handle) = events_handle {
        if let Some(events) = events.remove(events_handle.id) {
            log::debug!("loading events data");

            let mut game_events = HashMap::new();
            for event in events.0.into_iter() {
                game_events.insert(event.id.clone(), event);
            }

            commands.insert_resource(game_events);
            commands.remove_resource::<Handle<GameEvents>>()
        }
    }
}
