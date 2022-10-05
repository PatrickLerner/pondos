use super::Populations;
use bevy::prelude::*;

pub fn load_populations(
    mut commands: Commands,
    populations_handle: Option<Res<Handle<Populations>>>,
    mut populations: ResMut<Assets<Populations>>,
) {
    if let Some(populations_handle) = populations_handle {
        if let Some(populations) = populations.remove(populations_handle.id) {
            log::debug!("loading populations data");

            let mut populations = populations.0;
            populations.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

            commands.insert_resource(populations);
            commands.remove_resource::<Handle<Populations>>()
        }
    }
}
