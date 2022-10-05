use bevy::{prelude::*, reflect::TypeUuid, render::texture::ImageSettings};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use clap::Command;
use dotenv::dotenv;
use egui::{FontFamily, FontId, TextStyle};
use game_time::{GameTime, GameTimeAdvanceEvent};
use serde::Deserialize;
use std::collections::HashMap;

mod debug_populations;
mod game_time;
mod helpers;
mod loading;
mod map;
mod population;
mod settlement;

#[inline]
fn panel_heading() -> TextStyle {
    TextStyle::Name("PanelHeading".into())
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(panel_heading(), FontId::new(40.0, Proportional));
    ctx.set_style(style);
}

pub fn color_mode(mut egui_context: ResMut<EguiContext>) {
    configure_text_styles(egui_context.ctx_mut());
    egui_context.ctx_mut().set_visuals(egui::Visuals::light());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    Map,
    Settlement,
    TravelToSettlement,
    TradeWithSettlement,
}

#[derive(Default)]
pub struct Player {
    position: Vec2,
    location: Option<Entity>,
    location_marker: Option<Entity>,
    location_marker_texture_atlas_handle: Option<Handle<TextureAtlas>>,
    location_marker_need_update: bool,
    gold: u32,
    resources: HashMap<String, u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
// TODO: validate entries here when loading data
pub struct CalculatedPopulationValue(HashMap<String, f32>);

impl CalculatedPopulationValue {
    pub fn value(&self, populations: &Vec<String>) -> f32 {
        self.0.iter().fold(0.0, |acc, (pop, modifier)| {
            let count = if pop == "Population" {
                populations.len()
            } else {
                populations.iter().filter(|i| i == &pop).count()
            } as f32;

            acc + modifier * count
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, TypeUuid)]
#[serde(rename_all = "lowercase")]
#[uuid = "bafb929f-a7b1-45c3-b907-f71720724940"]
pub struct Settings {
    max_gold: CalculatedPopulationValue,
    max_multipliers: SeasonalAmount<f32>,
}

#[derive(Deserialize, Component, Debug, Hash, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SeasonalAmount<T> {
    #[serde(default)]
    pub growth: T,
    #[serde(default)]
    pub summer: T,
    #[serde(default)]
    pub harvest: T,
    #[serde(default)]
    pub winter: T,
}

impl<T> SeasonalAmount<T>
where
    T: Copy,
{
    pub fn value(&self, time: &GameTime) -> T {
        if time.is_growth_season() {
            self.growth
        } else if time.is_summer_season() {
            self.summer
        } else if time.is_harvest_season() {
            self.harvest
        } else if time.is_winter_season() {
            self.winter
        } else {
            unreachable!("unknown season");
        }
    }
}

pub struct PlayerTravelEvent {
    position: Vec2,
    entity: Entity,
}

impl PlayerTravelEvent {
    pub fn new(entity: Entity, x: u32, y: u32) -> Self {
        let position = Vec2::new(x as f32, y as f32);

        Self { position, entity }
    }
}

fn handle_travel(
    mut events: EventReader<PlayerTravelEvent>,
    mut player: ResMut<Player>,
    mut advance_time_events: EventWriter<GameTimeAdvanceEvent>,
) {
    for event in events.iter() {
        if player.position != event.position {
            log::info!(
                "Player traveled to {}:{}",
                event.position.x,
                event.position.y
            );
            player.position = event.position;
            player.location_marker_need_update = true;
            player.location = Some(event.entity);
            advance_time_events.send(GameTimeAdvanceEvent);
        }
    }
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

    match matches.subcommand() {
        Some(("debug_populations", _)) => {
            debug_populations::debug_populations();
            std::process::exit(0);
        }
        _ => {}
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
    .add_event::<PlayerTravelEvent>()
    .add_event::<game_time::GameTimeAdvancedEvent>()
    .add_event::<game_time::GameTimeAdvanceEvent>()
    .add_event::<settlement::CloseSettlementUIEvent>()
    .add_state(GameState::Loading)
    .insert_resource(ImageSettings::default_nearest())
    .init_resource::<game_time::GameTime>()
    .init_resource::<helpers::camera::GameCamera>()
    .init_resource::<Player>()
    .init_resource::<Option<settlement::SelectedSettlement>>()
    .add_plugins(DefaultPlugins)
    .add_plugin(YamlAssetPlugin::<loading::Settlements>::new(&[
        "settlements",
    ]))
    .add_plugin(YamlAssetPlugin::<settlement::Resources>::new(&[
        "resources",
    ]))
    .add_plugin(YamlAssetPlugin::<population::Populations>::new(&[
        "populations",
    ]))
    .add_plugin(YamlAssetPlugin::<Settings>::new(&["settings"]))
    .add_plugin(TilemapPlugin)
    .add_plugin(EguiPlugin)
    .add_plugin(loading::LoadingPlugin)
    .add_plugin(map::MapPlugin)
    .add_plugin(settlement::SettlementPlugin)
    .add_plugin(game_time::GameTimePlugin)
    .add_system(handle_travel)
    .add_system(population::population_production)
    .add_startup_system(color_mode)
    .add_startup_system(helpers::camera::spawn_camera)
    .run();
}
