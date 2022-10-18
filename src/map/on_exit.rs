use crate::types::Player;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::MapSize;

pub fn on_exit(
    mut commands: Commands,
    tilemap_query: Query<Entity, With<TileStorage>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut player: ResMut<Player>,
) {
    for entity in tilemap_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if let Some(entity) = player.location_marker {
        commands.entity(entity).despawn_recursive();
        player.location_marker = None;
    }

    if let Some(handle) = &player.location_marker_texture_atlas_handle {
        texture_atlases.remove(handle);
        player.location_marker_texture_atlas_handle = None;
    }

    commands.remove_resource::<MapSize>();
}
