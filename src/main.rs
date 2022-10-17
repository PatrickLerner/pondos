use std::collections::HashSet;

use bevy::{prelude::*, reflect::TypeUuid, render::texture::ImageSettings};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use clap::Command;
use dotenv::dotenv;
use serde::Deserialize;
use settlement::SettlementLabel;

mod building;
mod camera;
mod debug_populations;
mod debug_settlements;
mod deities;
mod game_events;
mod game_state;
mod game_time;
mod info_ui;
mod loading;
mod map;
mod player;
mod population;
mod price_calculator;
mod resources;
mod settlement;
mod trader;
mod types;
mod ui;

const COIN_NAME: &str = "Silver";

#[derive(Debug, Deserialize, PartialEq, TypeUuid)]
#[serde(rename_all = "lowercase")]
#[uuid = "bafb929f-a7b1-45c3-b907-f71720724940"]
pub struct Settings {
    max_silver: types::CalculatedPopulationValue,
    min_silver: types::CalculatedPopulationValue,
    start_settlement: String,
    start_silver: u32,
    max_multipliers: types::SeasonalAmount<f32>,
    cap_percentage: f32,
    events: HashSet<String>,
}

fn cli() -> Command {
    Command::new("pondos")
        .about("a game about trading")
        .subcommand(
            Command::new("debug")
                .subcommand(
                    Command::new("populations").about(
                        "Gives the yearly value each population brings to debug game balance",
                    ),
                )
                .subcommand(
                    Command::new("settlements").about(
                        "Gives the yearly value each settlement brings to debug game balance",
                    ),
                ),
        )
}

fn main() {
    dotenv().ok();

    let matches = cli().get_matches();

    if let Some(("debug", cmd)) = matches.subcommand() {
        if let Some(("populations", _)) = cmd.subcommand() {
            debug_populations::debug_populations();
        }
        if let Some(("settlements", _)) = cmd.subcommand() {
            debug_settlements::debug_settlements();
        }
        std::process::exit(0);
    }

    let mut app = App::new();

    #[cfg(not(target_family = "wasm"))]
    app.insert_resource(WindowDescriptor {
        width: 1680.0,
        height: 1050.0,
        title: String::from("Pondos"),
        ..Default::default()
    });
    #[cfg(target_family = "wasm")]
    app.insert_resource(WindowDescriptor {
        fit_canvas_to_parent: true,
        ..Default::default()
    });

    app.insert_resource(ClearColor(Color::rgb(
        204.0 / 255.0,
        197.0 / 255.0,
        185.0 / 255.0,
    )))
    .init_resource::<price_calculator::AveragePrices>()
    .add_event::<game_time::GameTimeAdvancedEvent>()
    .add_event::<game_time::GameTimeAdvanceEvent>()
    .add_event::<ui::CloseSettlementUIEvent>()
    .add_event::<settlement::VisitSettlementEvent>()
    .add_state(game_state::GameState::Map)
    .add_state(game_state::LoadingState::Loading)
    .add_state(game_state::RunningState::Running)
    .insert_resource(ImageSettings::default_nearest())
    .init_resource::<game_time::GameTime>()
    .add_plugins(DefaultPlugins)
    .add_plugin(YamlAssetPlugin::<loading::Settlements>::new(&[
        "settlements",
    ]))
    .add_plugin(YamlAssetPlugin::<loading::Resources>::new(&["resources"]))
    .add_plugin(YamlAssetPlugin::<loading::Populations>::new(&[
        "populations",
    ]))
    .add_plugin(YamlAssetPlugin::<loading::Deities>::new(&["deities"]))
    .add_plugin(YamlAssetPlugin::<loading::GameEvents>::new(&["events"]))
    .add_plugin(YamlAssetPlugin::<Settings>::new(&["settings"]))
    .add_plugin(TilemapPlugin)
    .add_plugin(EguiPlugin)
    .add_plugin(loading::LoadingPlugin)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(game_events::GameEventsPlugin)
    .add_plugin(map::MapPlugin)
    .add_plugin(settlement::SettlementPlugin)
    .add_plugin(building::BuildingPlugin)
    .add_plugin(game_time::GameTimePlugin)
    .add_system(population::population_production)
    .add_system(price_calculator::average_prices)
    .add_system(settlement::cap_resources::cap_resources.label(SettlementLabel::CapResources))
    .add_system(trader::trade_merchant.after(SettlementLabel::CapResources))
    .add_system(info_ui::info_ui)
    .add_startup_system(ui::color_mode)
    .add_startup_system(info_ui::show_game_version)
    .run();
}
