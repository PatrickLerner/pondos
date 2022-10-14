use crate::{loading::GameEvents, Settings};
use bevy::prelude::*;

pub fn load_settings(
    mut commands: Commands,
    settings_handle: Option<Res<Handle<Settings>>>,
    mut settings: ResMut<Assets<Settings>>,
    server: Res<AssetServer>,
) {
    if let Some(settings_handle) = settings_handle {
        if let Some(settings) = settings.remove(settings_handle.id) {
            log::debug!("loading settings data");

            let events: Vec<Handle<GameEvents>> = settings
                .events
                .iter()
                .map(|event_name| server.load(&format!("events/{}.events", event_name)))
                .collect();

            commands.insert_resource(events);
            commands.insert_resource(settings);
            commands.remove_resource::<Handle<Settings>>()
        }
    }
}
