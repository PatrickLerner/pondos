use super::{FeaturesTilemap, MapImage, Settlements};
use crate::{
    map::{constants::SETTLEMENT, MapSize},
    settlement::{BuildingType, Shipyard},
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn load_settlements(
    mut commands: Commands,
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

                        settlement.populations.sort();

                        for building in settlement.buildings.iter_mut() {
                            let building_component = match building.building_type {
                                BuildingType::Shipyard => Shipyard::default(),
                            };

                            let entity = commands.spawn().insert(building_component).id();

                            building.entity = entity;
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
