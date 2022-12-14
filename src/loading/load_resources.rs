use super::Resources;
use bevy::prelude::*;

pub fn load_resources(
    mut commands: Commands,
    resources_handle: Option<Res<Handle<Resources>>>,
    mut resources: ResMut<Assets<Resources>>,
) {
    if let Some(resources_handle) = resources_handle {
        if let Some(resources) = resources.remove(resources_handle.id) {
            log::debug!("loading resources data");

            let mut resources = resources.0;
            resources.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

            commands.insert_resource(resources);
            commands.remove_resource::<Handle<Resources>>()
        }
    }
}
