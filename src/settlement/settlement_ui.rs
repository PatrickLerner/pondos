use super::{CloseSettlementUIEvent, SelectedSettlement, Settlement};
use crate::{
    building::{BuildingType, SelectedBuilding},
    create_window_with_mobile,
    game_state::GameState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, Ui},
    EguiContext,
};

pub fn settlement_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_settlement: Res<Option<SelectedSettlement>>,
    mut selected_building: ResMut<Option<SelectedBuilding>>,
    settlements: Query<&Settlement>,
    mut events: EventWriter<CloseSettlementUIEvent>,
    mut game_state: ResMut<State<GameState>>,
    windows: Res<Windows>,
) {
    if let Some(entity) = selected_settlement.as_ref() {
        let settlement = settlements
            .get(entity.0)
            .expect("Expected settlement to be selected");

        let mut open = true;
        create_window_with_mobile(
            egui_context.ctx_mut(),
            &windows,
            &settlement.name,
            &mut open,
            |ui, mobile| {
                ui.add_space(10.);
                ui.with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                    let button = ui.add_sized([100., 30.], egui::Button::new("Trade"));
                    if button.clicked() {
                        game_state.push(GameState::TradeWithSettlement).unwrap();
                    }
                });

                if mobile {
                    ui.with_layout(egui::Layout::top_down_justified(Align::Min), |ui| {
                        egui::ScrollArea::both().id_source("info").show(ui, |ui| {
                            population_info(ui, settlement);
                            if !settlement.buildings.is_empty() {
                                ui.add_space(5.);
                                buildings_ui(
                                    ui,
                                    settlement,
                                    &mut game_state,
                                    &mut selected_building,
                                );
                            }
                            ui.add_space(5.);
                            resource_info(ui, settlement);
                        });
                    });
                } else {
                    ui.columns(2, |columns| {
                        egui::ScrollArea::vertical().id_source("population").show(
                            &mut columns[0],
                            |ui| {
                                population_info(ui, settlement);
                                if !settlement.buildings.is_empty() {
                                    ui.add_space(5.);
                                    buildings_ui(
                                        ui,
                                        settlement,
                                        &mut game_state,
                                        &mut selected_building,
                                    );
                                }
                            },
                        );

                        egui::ScrollArea::vertical()
                            .id_source("resources")
                            .show(&mut columns[1], |ui| resource_info(ui, settlement));
                    });
                }
            },
        );

        if !open {
            events.send(CloseSettlementUIEvent);
        }
    }
}

fn buildings_ui(
    ui: &mut Ui,
    settlement: &Settlement,
    game_state: &mut State<GameState>,
    selected_building: &mut Option<SelectedBuilding>,
) {
    ui.heading("Buildings");
    ui.add_space(5.);

    for building in settlement.buildings.iter() {
        match building.building_type {
            BuildingType::Shipyard => {
                if ui
                    .add_sized([100., 30.], egui::Button::new("Shipyard"))
                    .clicked()
                {
                    *selected_building = Some(SelectedBuilding(building.entity));
                    game_state.push(GameState::Shipyard).unwrap();
                }
            }
        }
    }
}

fn population_info(ui: &mut Ui, settlement: &Settlement) {
    ui.heading(format!("Population ({})", settlement.populations.len()));
    ui.add_space(5.);

    for population in settlement.populations.iter() {
        ui.label(population);
    }
}

fn resource_info(ui: &mut Ui, settlement: &Settlement) {
    ui.heading("Resources");
    ui.add_space(5.);

    let resources = &settlement.resources;
    let mut lines = vec![];
    for (resource, amount) in resources.iter().filter(|(_, amount)| *amount > &0) {
        lines.push(format!("{}: {}", resource, amount));
    }
    lines.sort();

    for line in lines {
        ui.label(line);
    }

    ui.label(format!("Silver: {}", settlement.silver));
}
