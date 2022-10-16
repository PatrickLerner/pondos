use super::GameEvents;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn load_events(
    mut commands: Commands,
    events_handles: Option<Res<Vec<Handle<GameEvents>>>>,
    mut events: ResMut<Assets<GameEvents>>,
) {
    if let Some(events_handles) = events_handles {
        log::debug!("loading events data");
        let mut game_events = HashMap::new();
        for events_handle in events_handles.iter() {
            if let Some(events) = events.remove(events_handle.id) {
                for event in events.0.into_iter() {
                    game_events.insert(event.id.clone(), event);
                }
            }
        }

        log::debug!("loaded {} events", game_events.len());
        commands.insert_resource(game_events);
        commands.remove_resource::<Vec<Handle<GameEvents>>>()
    }
}
