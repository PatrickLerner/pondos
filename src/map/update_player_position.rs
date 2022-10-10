use crate::{
    map::{
        constants::{PLAYER_MARKER, TILEMAP_COLUMNS, TILEMAP_ROWS, TILEMAP_SIZE, Z_MARKER},
        MapSize,
    },
    player::Player,
};
use bevy::prelude::*;

pub fn update_player_position(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Option<ResMut<Player>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    map_size: Option<Res<MapSize>>,
) {
    if player.is_none() || map_size.is_none() {
        return;
    };
    let mut player = player.unwrap();
    let map_size = map_size.unwrap();

    if !player.location_marker_need_update {
        return;
    }

    let entity = player
        .location_marker
        .unwrap_or_else(|| commands.spawn().id());

    let texture_atlas_handle = player
        .location_marker_texture_atlas_handle
        .clone()
        .unwrap_or_else(|| {
            let texture_handle: Handle<Image> = asset_server.load("tiles.png");

            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::splat(TILEMAP_SIZE),
                TILEMAP_COLUMNS,
                TILEMAP_ROWS,
            );
            texture_atlases.add(texture_atlas)
        });

    commands.entity(entity).insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: PLAYER_MARKER.0 as usize,
            ..default()
        },
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_xyz(
            player.position.x * TILEMAP_SIZE,
            (map_size.height as f32 - 1.0 - player.position.y + 1.0) * TILEMAP_SIZE,
            Z_MARKER,
        ),
        ..default()
    });

    player.location_marker = Some(entity);
    player.location_marker_texture_atlas_handle = Some(texture_atlas_handle);
    player.location_marker_need_update = false;
    log::debug!("updated player marker");
}
