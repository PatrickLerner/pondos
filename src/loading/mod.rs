use crate::{
    population::Populations,
    settlement::{Resources, Settlement},
    GameState, Player,
};
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;

mod initialize_settlements;
mod load_map;
mod load_player;
mod load_populations;
mod load_resources;
mod load_settlements;

#[derive(Default)]
pub struct AssetsLoading(Vec<HandleUntyped>);

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct Settlements(Vec<Settlement>);

pub struct MapImage(Handle<Image>);
pub struct FeaturesTilemap((Entity, TilemapId));

#[derive(Component)]
pub struct RequiresInitialization;

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let map_image: Handle<Image> = server.load("map.png");
    let map_image = MapImage(map_image);
    let settlements: Handle<Settlements> = server.load("game.settlements");
    let resources: Handle<Resources> = server.load("game.resources");
    let populations: Handle<Populations> = server.load("game.populations");

    log::debug!("requesting resources");
    commands.insert_resource(map_image);
    commands.insert_resource(settlements);
    commands.insert_resource(resources);
    commands.insert_resource(populations);
}

pub fn transition(
    mut game_state: ResMut<State<GameState>>,
    settlement_handle: Option<Res<Handle<Settlements>>>,
    populations_handle: Option<Res<Handle<Populations>>>,
    resources_handle: Option<Res<Handle<Resources>>>,
    map_image_handle: Option<Res<MapImage>>,
    player: Option<Res<Player>>,
    uninitialized_settlements: Query<(), (With<Settlement>, With<RequiresInitialization>)>,
) {
    if settlement_handle.is_none()
        && populations_handle.is_none()
        && map_image_handle.is_none()
        && resources_handle.is_none()
        && player.is_some()
        && uninitialized_settlements.iter().any(|_| true)
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
                    .with_system(initialize_settlements::initialize_settlements)
                    .with_system(load_player::load_player),
            );
    }
}
