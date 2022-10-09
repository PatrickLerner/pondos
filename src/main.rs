use bevy::{prelude::*, reflect::TypeUuid, render::texture::ImageSettings};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use clap::Command;
use dotenv::dotenv;
use serde::Deserialize;

mod building;
mod camera;
mod debug_populations;
mod debug_settlements;
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

const COIN_NAME: &str = "Silver";

#[derive(Debug, Deserialize, PartialEq, TypeUuid)]
#[serde(rename_all = "lowercase")]
#[uuid = "bafb929f-a7b1-45c3-b907-f71720724940"]
pub struct Settings {
    max_silver: types::CalculatedPopulationValue,
    min_silver: types::CalculatedPopulationValue,
    max_multipliers: types::SeasonalAmount<f32>,
    cap_percentage: f32,
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

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
fn init_game_version(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert_bundle(
        TextBundle::from_section(
            format!("{} v{}", NAME, VERSION),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 14.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    );
}

const WINDOW_PADDING_X: f32 = 40.;
const WINDOW_PADDING_Y: f32 = 80.;
const MAX_WIDTH: f32 = 720.;
const MAX_HEIGHT: f32 = 720.;
const MOBILE_BREAK_POINT: f32 = 400.;

pub fn create_window<'a>(
    ctx: &bevy_egui::egui::Context,
    windows: &'a Windows,
    name: &str,
    open: &mut bool,
    add_contents: impl FnOnce(&mut bevy_egui::egui::Ui),
) {
    create_window_with_mobile(ctx, windows, name, open, |ui, _| add_contents(ui))
}

pub fn create_window_with_mobile<'a>(
    ctx: &bevy_egui::egui::Context,
    windows: &'a Windows,
    name: &str,
    open: &mut bool,
    add_contents: impl FnOnce(&mut bevy_egui::egui::Ui, bool),
) {
    let window = windows.get_primary().unwrap();
    let win_max_width = window.width() - WINDOW_PADDING_X;
    let width = f32::min(win_max_width, MAX_WIDTH);
    let win_max_height = window.height() - WINDOW_PADDING_Y;
    let height = f32::min(win_max_height, MAX_HEIGHT);

    bevy_egui::egui::Window::new(name)
        .anchor(bevy_egui::egui::Align2::CENTER_CENTER, (0., 0.))
        .resizable(false)
        .collapsible(false)
        .open(open)
        .show(ctx, |ui| {
            ui.set_width(width);
            ui.set_height(height);

            let mobile = win_max_width <= MOBILE_BREAK_POINT;
            add_contents(ui, mobile);
        });
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
    .add_event::<player::PlayerTravelEvent>()
    .add_event::<game_time::GameTimeAdvancedEvent>()
    .add_event::<game_time::GameTimeAdvanceEvent>()
    .add_event::<settlement::CloseSettlementUIEvent>()
    .add_state(game_state::GameState::Loading)
    .insert_resource(ImageSettings::default_nearest())
    .init_resource::<game_time::GameTime>()
    .init_resource::<Option<settlement::SelectedSettlement>>()
    .init_resource::<Option<building::SelectedBuilding>>()
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
    .add_plugin(building::BuildingPlugin)
    .add_plugin(game_time::GameTimePlugin)
    .add_system(player::handle_travel)
    .add_system(population::population_production)
    .add_system(price_calculator::average_prices)
    .add_startup_system(ui_config::color_mode)
    .add_startup_system(init_game_version)
    .run();
}
