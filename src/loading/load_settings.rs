use crate::Settings;
use bevy::prelude::*;

pub fn load_settings(
    mut commands: Commands,
    settings_handle: Option<Res<Handle<Settings>>>,
    mut settings: ResMut<Assets<Settings>>,
) {
    if let Some(settings_handle) = settings_handle {
        if let Some(settings) = settings.remove(settings_handle.id) {
            log::debug!("loading settings data");
            commands.insert_resource(settings);
            commands.remove_resource::<Handle<Settings>>()
        }
    }
}
