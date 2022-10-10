use super::{FeaturesTilemap, MapImage};
use crate::{
    camera::GameCamera,
    map::{
        constants::{TILEMAP_SIZE, Z_FEATURES, Z_GROUND},
        types::{MapTileType, Overlay},
        MapSize,
    },
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

struct MapLoader<'a> {
    map_image: &'a Image,
    map_size: &'a MapSize,
}

impl<'a> MapLoader<'a> {
    pub fn map_tile(&self, x: i32, y: i32) -> Option<MapTileType> {
        if y < 0 || x < 0 || y >= self.map_size.height as i32 || x >= self.map_size.width as i32 {
            return None;
        }

        let offset = (4 * (x + y * self.map_size.width as i32)) as usize;
        let pixel = (
            self.map_image.data[offset],
            self.map_image.data[offset + 1],
            self.map_image.data[offset + 2],
        );

        Some(match pixel {
            (137, 249, 79) => MapTileType::Grass,
            (0, 0, 255) => MapTileType::Water,
            (93, 63, 20) => MapTileType::Mountain,
            (255, 148, 0) => MapTileType::Hills,
            (4, 113, 1) => MapTileType::Woods,
            _ => panic!("Unknown color on map {:?}", pixel),
        })
    }

    pub fn map_tile_with_mapping(&self, x: i32, y: i32) -> Option<MapTileType> {
        self.map_tile(x, y).map(|map_tile| {
            if MapTileType::Water == map_tile {
                let left = self.map_tile(x - 1, y);
                let left = left.is_none() || left == Some(MapTileType::Water);
                let right = self.map_tile(x + 1, y);
                let right = right.is_none() || right == Some(MapTileType::Water);
                let top = self.map_tile(x, y - 1);
                let top = top.is_none() || top == Some(MapTileType::Water);
                let bottom = self.map_tile(x, y + 1);
                let bottom = bottom.is_none() || bottom == Some(MapTileType::Water);

                if (top == bottom) && (left == right) && (top == right) {
                    MapTileType::Water
                } else {
                    MapTileType::GrassOverlay(Overlay {
                        right,
                        left,
                        top,
                        bottom,
                    })
                }
            } else {
                map_tile
            }
        })
    }
}

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

            let map_loader = MapLoader {
                map_image: &map_image,
                map_size: &map_size,
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
                    let map_tile_type = map_loader
                        .map_tile_with_mapping(x as i32, y as i32)
                        .unwrap();

                    let position = TilePos {
                        x,
                        y: map_size.height - 1 - y,
                    };

                    let ground_tile = if map_tile_type.ground() {
                        map_tile_type
                    } else {
                        map_tile_type.ground_tile()
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

                        if ground_tile.animation_count() > 1 {
                            commands.entity(tile_entity).insert(AnimatedTile {
                                start: texture.0,
                                end: texture.0 + ground_tile.animation_count(),
                                speed: 0.3,
                            });
                        }

                        ground_tile_storage.set(&position, tile_entity);
                    }

                    if !map_tile_type.ground() {
                        let texture = map_tile_type.texture(winter);

                        let flip = map_tile_type.flip().unwrap_or_default();

                        let tile_entity = commands
                            .spawn()
                            .insert_bundle(TileBundle {
                                position,
                                tilemap_id: features_tilemap_id,
                                flip,
                                texture,
                                ..Default::default()
                            })
                            .insert(map_tile_type)
                            .id();

                        features_tile_storage.set(&position, tile_entity);
                    }
                }
            }

            commands
                .entity(ground_tilemap_entity)
                .insert_bundle(TilemapBundle {
                    grid_size: tile_size.into(),
                    size: map_size.into(),
                    storage: ground_tile_storage,
                    texture: TilemapTexture::Single(texture_handle.clone()),
                    tile_size,
                    map_type: TilemapType::Square {
                        diagonal_neighbors: false,
                    },
                    transform: Transform::from_xyz(0.0, 0.0, Z_GROUND),
                    ..Default::default()
                });

            commands
                .entity(features_tilemap_entity)
                .insert_bundle(TilemapBundle {
                    grid_size: tile_size.into(),
                    size: map_size.into(),
                    storage: features_tile_storage,
                    texture: TilemapTexture::Single(texture_handle),
                    tile_size,
                    map_type: TilemapType::Square {
                        diagonal_neighbors: false,
                    },
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
