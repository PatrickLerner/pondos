use std::collections::HashMap;

use bevy::{prelude::*, reflect::TypeUuid, render::texture::ImageSettings};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use clap::Command;
use dotenv::dotenv;
use game_time::GameTimeAdvancedEvent;
use price_calculator::PriceCalculator;
use serde::Deserialize;

mod camera;
mod debug_populations;
mod game_state;
mod game_time;
mod loading;
mod map;
mod player;
mod population;
mod price_calculator;
mod settlement;
mod types;
mod ui_config;

pub use game_state::GameState;
pub use player::Player;
use settlement::{Resource, Settlement};

#[derive(Debug, Deserialize, PartialEq, TypeUuid)]
#[serde(rename_all = "lowercase")]
#[uuid = "bafb929f-a7b1-45c3-b907-f71720724940"]
pub struct Settings {
    max_gold: types::CalculatedPopulationValue,
    max_multipliers: types::SeasonalAmount<f32>,
}

fn cli() -> Command {
    Command::new("pondos")
        .about("a game about trading")
        .subcommand(
            Command::new("debug_populations")
                .about("Gives the yearly value each population brings to debug game balance"),
        )
}

#[derive(Default)]
pub struct AveragePrices {
    pub prices: HashMap<String, f32>,
}

fn average_prices(
    settlements: Query<&Settlement>,
    mut average_prices: ResMut<AveragePrices>,
    resources: Option<Res<Vec<Resource>>>,
    events: EventReader<GameTimeAdvancedEvent>,
) {
    if events.is_empty() || resources.is_none() {
        return;
    }

    let resources = resources.unwrap();

    let settlement_count = settlements.iter().len() as f32;

    for resource in resources.iter() {
        let sum = settlements.iter().fold(0.0, |acc, settlement| {
            let demand = resource.demand.value(&settlement.populations).ceil() as u32;

            let prices = PriceCalculator {
                base_price: resource.base_price,
                demand,
                supply: *settlement.resources.get(&resource.name).unwrap_or(&0),
            };

            acc + prices.sell_price() as f32
        });

        *average_prices
            .prices
            .entry(resource.name.clone())
            .or_default() = sum / settlement_count;
    }
}

fn main() {
    dotenv().ok();

    let matches = cli().get_matches();

    if let Some(("debug_populations", _)) = matches.subcommand() {
        debug_populations::debug_populations();
        std::process::exit(0);
    }

    let mut app = App::new();

    #[cfg(not(target_family = "wasm"))]
    app.insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
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
    .init_resource::<AveragePrices>()
    .add_event::<player::PlayerTravelEvent>()
    .add_event::<game_time::GameTimeAdvancedEvent>()
    .add_event::<game_time::GameTimeAdvanceEvent>()
    .add_event::<settlement::CloseSettlementUIEvent>()
    .add_state(GameState::Loading)
    .insert_resource(ImageSettings::default_nearest())
    .init_resource::<game_time::GameTime>()
    .init_resource::<Player>()
    .init_resource::<Option<settlement::SelectedSettlement>>()
    .add_plugins(DefaultPlugins)
    .add_plugin(YamlAssetPlugin::<loading::Settlements>::new(&[
        "settlements",
    ]))
    .add_plugin(YamlAssetPlugin::<loading::Resources>::new(&["resources"]))
    .add_plugin(YamlAssetPlugin::<loading::Populations>::new(&[
        "populations",
    ]))
    .add_plugin(YamlAssetPlugin::<Settings>::new(&["settings"]))
    .add_plugin(TilemapPlugin)
    .add_plugin(EguiPlugin)
    .add_plugin(loading::LoadingPlugin)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(map::MapPlugin)
    .add_plugin(settlement::SettlementPlugin)
    .add_plugin(game_time::GameTimePlugin)
    .add_system(player::handle_travel)
    .add_system(population::population_production)
    .add_system(average_prices)
    .add_startup_system(ui_config::color_mode)
    .run();
}
