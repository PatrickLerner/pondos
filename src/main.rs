use bevy::{prelude::*, reflect::TypeUuid, render::texture::ImageSettings};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use clap::Command;
use dotenv::dotenv;
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
mod resources;
mod settlement;
mod types;
mod ui_config;

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
    .init_resource::<price_calculator::AveragePrices>()
    .add_event::<player::PlayerTravelEvent>()
    .add_event::<game_time::GameTimeAdvancedEvent>()
    .add_event::<game_time::GameTimeAdvanceEvent>()
    .add_event::<settlement::CloseSettlementUIEvent>()
    .add_state(game_state::GameState::Loading)
    .insert_resource(ImageSettings::default_nearest())
    .init_resource::<game_time::GameTime>()
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
    .add_system(price_calculator::average_prices)
    .add_startup_system(ui_config::color_mode)
    .run();
}
