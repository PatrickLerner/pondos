use crate::{
    deities::Deity,
    game_events::GameEvent,
    game_state::LoadingState,
    game_time::GameTime,
    population::Population,
    resources::Resource,
    types::{Player, Settlement},
    Settings,
};
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;

mod initialize_game_time;
mod load_deities;
mod load_events;
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

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "713da916-235c-4b20-912b-daccf93f99d1"]
pub struct Deities(Vec<Deity>);

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "599d5626-6452-49b3-b5a1-7b3292071509"]
pub struct GameEvents(Vec<GameEvent>);

pub struct MapImage(Handle<Image>);
pub struct FeaturesTilemap((Entity, TilemapId));

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let map_image: Handle<Image> = server.load("map.png");
    let map_image = MapImage(map_image);
    let settlements: Handle<Settlements> = server.load("game.settlements");
    let resources: Handle<Resources> = server.load("game.resources");
    let populations: Handle<Populations> = server.load("game.populations");
    let settings: Handle<Settings> = server.load("game.settings");
    let deities: Handle<Deities> = server.load("game.deities");

    log::debug!("requesting assets");
    commands.insert_resource(map_image);
    commands.insert_resource(settlements);
    commands.insert_resource(resources);
    commands.insert_resource(populations);
    commands.insert_resource(deities);
    commands.insert_resource(settings);
}

#[allow(clippy::type_complexity)]
pub fn transition(
    mut loading_state: ResMut<State<LoadingState>>,
    res: (
        Option<Res<Handle<Settlements>>>,
        Option<Res<Handle<Populations>>>,
        Option<Res<Handle<Deities>>>,
        Option<Res<MapImage>>,
        Option<Res<Handle<Resources>>>,
        Option<Res<Handle<Settings>>>,
        Option<Res<Vec<Handle<GameEvents>>>>,
        Res<GameTime>,
    ),
    player: Option<Res<Player>>,
) {
    let (
        settlement_handle,
        populations_handle,
        deities_handle,
        map_image_handle,
        resources_handle,
        settings_handle,
        events_handle,
        game_time,
    ) = res;

    if settlement_handle.is_none()
        && populations_handle.is_none()
        && deities_handle.is_none()
        && map_image_handle.is_none()
        && resources_handle.is_none()
        && settings_handle.is_none()
        && events_handle.is_none()
        && player.is_some()
        && game_time.is_initialized()
    {
        log::info!("all resources fully loaded");
        loading_state.set(LoadingState::Loaded).unwrap();
    }
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(LoadingState::Loading).with_system(setup))
            .add_system_set(
                SystemSet::on_update(LoadingState::Loading)
                    .with_system(transition)
                    .with_system(load_map::load_map)
                    .with_system(load_settlements::load_settlements)
                    .with_system(load_populations::load_populations)
                    .with_system(load_resources::load_resources)
                    .with_system(load_deities::load_deities)
                    .with_system(load_events::load_events)
                    .with_system(load_settings::load_settings)
                    .with_system(initialize_game_time::initialize_game_time)
                    .with_system(load_player::load_player),
            );
    }
}
