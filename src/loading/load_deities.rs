use super::Deities;
use bevy::prelude::*;

pub fn load_deities(
    mut commands: Commands,
    deities_handle: Option<Res<Handle<Deities>>>,
    mut deities: ResMut<Assets<Deities>>,
) {
    if let Some(deities_handle) = deities_handle {
        if let Some(deities) = deities.remove(deities_handle.id) {
            log::debug!("loading deities data");

            commands.insert_resource(deities);
            commands.remove_resource::<Handle<Deities>>()
        }
    }
}
