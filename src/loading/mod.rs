use crate::{
    game_time::GameTime,
    population::Population,
    settlement::{Resource, Settlement},
    GameState, Player, Settings,
};
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;

mod initialize_game_time;
mod load_map;
mod load_player;
mod load_populations;
mod load_resources;
mod load_settings;
mod load_settlements;

#[derive(Default)]
pub struct AssetsLoading(Vec<HandleUntyped>);

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct Settlements(Vec<Settlement>);

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "b8c204ad-f39e-4358-a88b-24d2c342140f"]
pub struct Resources(Vec<Resource>);

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "e7ac1c59-c2ac-4a77-9ace-532038a44758"]
pub struct Populations(Vec<Population>);

pub struct MapImage(Handle<Image>);
pub struct FeaturesTilemap((Entity, TilemapId));

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let map_image: Handle<Image> = server.load("map.png");
    let map_image = MapImage(map_image);
    let settlements: Handle<Settlements> = server.load("game.settlements");
    let resources: Handle<Resources> = server.load("game.resources");
    let populations: Handle<Populations> = server.load("game.populations");
    let settings: Handle<Settings> = server.load("game.settings");

    log::debug!("requesting assets");
    commands.insert_resource(map_image);
    commands.insert_resource(settlements);
    commands.insert_resource(resources);
    commands.insert_resource(populations);
    commands.insert_resource(settings);
}

#[allow(clippy::type_complexity)]
pub fn transition(
    mut game_state: ResMut<State<GameState>>,
    res: (
        Option<Res<Handle<Settlements>>>,
        Option<Res<Handle<Populations>>>,
        Option<Res<MapImage>>,
        Option<Res<Handle<Resources>>>,
        Option<Res<Handle<Settings>>>,
        Res<GameTime>,
    ),
    player: Option<Res<Player>>,
) {
    let (
        settlement_handle,
        populations_handle,
        map_image_handle,
        resources_handle,
        settings_handle,
        game_time,
    ) = res;

    if settlement_handle.is_none()
        && populations_handle.is_none()
        && map_image_handle.is_none()
        && resources_handle.is_none()
        && settings_handle.is_none()
        && player.is_some()
        && game_time.is_initialized()
    {
        log::info!("all resources fully loaded");
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
                    .with_system(load_settlements::load_settlements)
                    .with_system(load_populations::load_populations)
                    .with_system(load_resources::load_resources)
                    .with_system(load_settings::load_settings)
                    .with_system(initialize_game_time::initialize_game_time)
                    .with_system(load_player::load_player),
            );
    }
}
