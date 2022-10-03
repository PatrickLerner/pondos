use super::{GRASS, HILLS, MOUNTAIN, SETTLEMENT, TILEMAP_SIZE, WATER, WOODS, Z_FEATURES, Z_GROUND};
use crate::{
    game_time::GameTime, helpers::camera::GameCamera, map::MapSize, settlement::Settlement,
    GameState, Player,
};
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;

#[derive(Default)]
pub struct AssetsLoading(Vec<HandleUntyped>);

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct Settlements(Vec<Settlement>);

pub struct MapImage(Handle<Image>);
pub struct FeaturesTilemap((Entity, TilemapId));

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let map_image: Handle<Image> = server.load("map.png");
    let map_image = MapImage(map_image);
    let settlements: Handle<Settlements> = server.load("settlements.yml");

    log::debug!("requesting map resources");
    commands.insert_resource(map_image);
    commands.insert_resource(settlements);
}

pub fn transition(
    mut game_state: ResMut<State<GameState>>,
    settlement_handle: Option<Res<Handle<Settlements>>>,
    map_image_handle: Option<Res<MapImage>>,
) {
    if settlement_handle.is_none() && map_image_handle.is_none() {
        log::info!("map fully loaded");
        game_state.set(GameState::Map).unwrap();
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

            let mut ground_tile_storage = TileStorage::empty(map_size.into());
            let ground_tilemap_entity = commands.spawn().id();
            let ground_tilemap_id = TilemapId(ground_tilemap_entity);

            let mut features_tile_storage = TileStorage::empty(map_size.into());
            let features_tilemap_entity = commands.spawn().id();
            let features_tilemap_id = TilemapId(features_tilemap_entity);

            fill_tilemap_rect(
                GRASS,
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

                    let (tile_storage, tilemap_id, texture) = match pixel {
                        (137, 249, 79) => (&mut ground_tile_storage, ground_tilemap_id, GRASS),
                        (0, 0, 255) => (&mut ground_tile_storage, ground_tilemap_id, WATER),
                        (93, 63, 20) => (&mut features_tile_storage, features_tilemap_id, MOUNTAIN),
                        (255, 148, 0) => (&mut features_tile_storage, features_tilemap_id, HILLS),
                        (4, 113, 1) => (&mut features_tile_storage, features_tilemap_id, WOODS),
                        _ => panic!("Unknown color on map {:?}", pixel),
                    };

                    let position = TilePos {
                        x,
                        y: map_size.height - 1 - y,
                    };

                    let tile_entity = commands
                        .spawn()
                        .insert_bundle(TileBundle {
                            position,
                            tilemap_id,
                            texture,
                            ..Default::default()
                        })
                        .id();
                    tile_storage.set(&position, Some(tile_entity));
                }
            }

            let tile_size = TilemapTileSize {
                x: TILEMAP_SIZE,
                y: TILEMAP_SIZE,
            };

            game_camera.position = Vec2::new(
                tile_size.x * map_size.width as f32 / 2.0,
                tile_size.y * map_size.height as f32 / 2.0,
            );

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

pub fn load_settlements(
    mut commands: Commands,
    mut player: ResMut<Player>,
    settlement_handle: Option<Res<Handle<Settlements>>>,
    mut settlements: ResMut<Assets<Settlements>>,
    map_size: Option<Res<MapSize>>,
    features_tilemap_id: Option<Res<FeaturesTilemap>>,
    mut tilemap_query: Query<&mut TileStorage>,
) {
    if let Some(settlement_handle) = settlement_handle {
        if let Some(map_size) = map_size {
            if let Some(features_tilemap_id) = features_tilemap_id {
                if settlements.get(settlement_handle.as_ref()).is_some() {
                    log::debug!("loading settlements data");
                    let settlements = settlements.remove(settlement_handle.id).unwrap();

                    for mut settlement in settlements.0.into_iter() {
                        let position = TilePos {
                            x: settlement.position.x,
                            y: map_size.height - 1 - settlement.position.y,
                        };

                        player.position =
                            Vec2::new(settlement.position.x as f32, settlement.position.y as f32);
                        player.location_marker_need_update = true;

                        let mut time = GameTime { year: 0, season: 0 };
                        for _ in 0..20 {
                            settlement.production_tick(&time);
                            settlement.resource_cap_tick(&time);
                            time.advance();
                        }

                        let tile_entity = commands
                            .spawn()
                            .insert_bundle(TileBundle {
                                position,
                                tilemap_id: features_tilemap_id.0 .1,
                                texture: SETTLEMENT,
                                ..Default::default()
                            })
                            .insert(settlement)
                            .id();

                        player.location = Some(tile_entity);

                        let mut features_tile_storage =
                            tilemap_query.get_mut(features_tilemap_id.0 .0).unwrap();
                        features_tile_storage.set(&position, Some(tile_entity));
                    }

                    commands.remove_resource::<MapImage>();
                    commands.remove_resource::<Handle<Settlements>>()
                }
            }
        }
    }
}
