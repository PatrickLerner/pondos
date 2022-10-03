use crate::{settlement::Settlement, GameState};
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;

mod load_map;
mod load_settlements;

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

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Loading)
                    .with_system(transition)
                    .with_system(load_map::load_map)
                    .with_system(load_settlements::load_settlements),
            );
    }
}
