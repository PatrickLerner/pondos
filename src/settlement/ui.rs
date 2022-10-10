use super::Settlement;
use crate::{
    building::{BuildingType, SelectedBuilding},
    game_state::{GameState, SettlementState},
    ui_config::large_button,
    COIN_NAME,
};
use bevy::prelude::*;
use bevy_egui::egui::Ui;

pub fn production_ui(ui: &mut Ui, settlement: &Settlement) {
    ui.heading("Local production");
    ui.add_space(5.);

    for item in settlement.produced_items() {
        ui.label(format!(" - {}", item));
    }
}

pub fn population_info(ui: &mut Ui, settlement: &Settlement) {
    ui.heading(format!("Population ({})", settlement.populations.len()));
    ui.add_space(5.);

    for population in settlement.populations.iter() {
        ui.label(format!(" - {}", population));
    }
}

pub fn resource_info(ui: &mut Ui, settlement: &Settlement) {
    ui.heading("Resources");
    ui.add_space(5.);

    let resources = &settlement.resources;
    let mut lines = vec![];
    for (resource, amount) in resources.iter().filter(|(_, amount)| *amount > &0) {
        lines.push(format!(" - {}: {}", resource, amount));
    }
    lines.sort();

    for line in lines {
        ui.label(line);
    }

    ui.label(format!(" - {}: {}", COIN_NAME, settlement.silver));
}

pub fn buildings_ui(
    ui: &mut Ui,
    settlement: &Settlement,
    game_state: &mut State<GameState>,
    selected_building: &mut Option<SelectedBuilding>,
) {
    ui.heading("Buildings");
    ui.add_space(5.);

    if large_button(ui, 100., "Market").clicked() {
        game_state
            .set(GameState::Settlement(SettlementState::Trade))
            .unwrap();
    }

    for building in settlement.buildings.iter() {
        match building.building_type {
            BuildingType::Shipyard => {
                if large_button(ui, 100., "Shipyard").clicked() {
                    *selected_building = Some(SelectedBuilding(building.entity));
                    game_state
                        .set(GameState::Settlement(SettlementState::Shipyard))
                        .unwrap();
                }
            }
        }
    }
}
