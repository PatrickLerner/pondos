use super::{FeaturesTilemap, MapImage};
use crate::{
    camera::GameCamera,
    map::{
        constants::{TILEMAP_SIZE, Z_FEATURES, Z_GROUND},
        types::MapTileType,
        MapSize,
    },
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_camera: ResMut<GameCamera>,
    map_image_handle: Option<Res<MapImage>>,
    mut map_image: ResMut<Assets<Image>>,
) {
    if let Some(map_image_handle) = map_image_handle {
        if map_image.get(&map_image_handle.0).is_some() {
            log::debug!("loading terrain data");

            let map_image = map_image.remove(map_image_handle.0.id).unwrap();

            let texture_handle: Handle<Image> = asset_server.load("tiles.png");

            let size = map_image.size();

            log::info!("loaded map {}x{}", size.x, size.y);

            let map_size = MapSize {
                width: size.x as u32,
                height: size.y as u32,
            };
            let tile_size = TilemapTileSize {
                x: TILEMAP_SIZE,
                y: TILEMAP_SIZE,
            };

            game_camera.pan_max = Vec2::new(
                tile_size.x * map_size.width as f32,
                tile_size.y * map_size.height as f32,
            );
            game_camera.position = game_camera.pan_max / 2.0;

            let mut ground_tile_storage = TileStorage::empty(map_size.into());
            let ground_tilemap_entity = commands.spawn().id();
            let ground_tilemap_id = TilemapId(ground_tilemap_entity);

            let mut features_tile_storage = TileStorage::empty(map_size.into());
            let features_tilemap_entity = commands.spawn().id();
            let features_tilemap_id = TilemapId(features_tilemap_entity);

            let winter = false;
            fill_tilemap_rect(
                MapTileType::Grass.texture(winter),
                TilePos { x: 0, y: 0 },
                map_size.into(),
                ground_tilemap_id,
                &mut commands,
                &mut ground_tile_storage,
            );

            for x in 0..map_size.width {
                for y in 0..map_size.height {
                    let offset = (4 * (x + y * map_size.width)) as usize;
                    let pixel = (
                        map_image.data[offset],
                        map_image.data[offset + 1],
                        map_image.data[offset + 2],
                    );

                    let map_tile_type = match pixel {
                        (137, 249, 79) => MapTileType::Grass,
                        (0, 0, 255) => MapTileType::Water,
                        (93, 63, 20) => MapTileType::Mountain,
                        (255, 148, 0) => MapTileType::Hills,
                        (4, 113, 1) => MapTileType::Woods,
                        _ => panic!("Unknown color on map {:?}", pixel),
                    };

                    let position = TilePos {
                        x,
                        y: map_size.height - 1 - y,
                    };

                    let ground_tile = if map_tile_type.ground() {
                        map_tile_type
                    } else {
                        MapTileType::Grass
                    };

                    {
                        let texture = ground_tile.texture(winter);
                        let tile_entity = commands
                            .spawn()
                            .insert_bundle(TileBundle {
                                position,
                                tilemap_id: ground_tilemap_id,
                                texture,
                                ..Default::default()
                            })
                            .insert(ground_tile)
                            .id();

                        if map_tile_type.animation_count() > 1 {
                            commands.entity(tile_entity).insert(AnimatedTile {
                                start: texture.0,
                                end: texture.0 + map_tile_type.animation_count(),
                                speed: 0.3,
                            });
                        }

                        ground_tile_storage.set(&position, Some(tile_entity));
                    }

                    if !map_tile_type.ground() {
                        let texture = map_tile_type.texture(winter);
                        let tile_entity = commands
                            .spawn()
                            .insert_bundle(TileBundle {
                                position,
                                tilemap_id: features_tilemap_id,
                                texture,
                                ..Default::default()
                            })
                            .insert(map_tile_type)
                            .id();

                        features_tile_storage.set(&position, Some(tile_entity));
                    }
                }
            }

            commands
                .entity(ground_tilemap_entity)
                .insert_bundle(TilemapBundle {
                    grid_size: tile_size.into(),
                    size: map_size.into(),
                    storage: ground_tile_storage,
                    texture: TilemapTexture(texture_handle.clone()),
                    tile_size,
                    mesh_type: TilemapMeshType::Square,
                    transform: Transform::from_xyz(0.0, 0.0, Z_GROUND),
                    ..Default::default()
                });

            commands
                .entity(features_tilemap_entity)
                .insert_bundle(TilemapBundle {
                    grid_size: tile_size.into(),
                    size: map_size.into(),
                    storage: features_tile_storage,
                    texture: TilemapTexture(texture_handle),
                    tile_size,
                    mesh_type: TilemapMeshType::Square,
                    transform: Transform::from_xyz(0.0, 0.0, Z_FEATURES),
                    ..Default::default()
                });

            commands.insert_resource(FeaturesTilemap((
                features_tilemap_entity,
                features_tilemap_id,
            )));
            commands.insert_resource(map_size);

            commands.remove_resource::<MapImage>();
        }
    }
}
