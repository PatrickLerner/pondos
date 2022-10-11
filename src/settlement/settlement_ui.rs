use crate::{
    game_state::GameState,
    settlement::{
        ui::{buildings_ui, population_info, production_ui, resource_info},
        Settlement,
    },
    ui::{create_window_with_mobile, CloseSettlementUIEvent, SelectedBuilding, SelectedSettlement},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align},
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

                if mobile {
                    ui.with_layout(egui::Layout::top_down_justified(Align::Min), |ui| {
                        egui::ScrollArea::both().id_source("info").show(ui, |ui| {
                            buildings_ui(ui, settlement, &mut game_state, &mut selected_building);
                            ui.add_space(5.);
                            population_info(ui, settlement);
                            ui.add_space(5.);
                            resource_info(ui, settlement);
                            ui.add_space(5.);
                            production_ui(ui, settlement);
                        });
                    });
                } else {
                    ui.columns(2, |columns| {
                        egui::ScrollArea::vertical().id_source("population").show(
                            &mut columns[0],
                            |ui| {
                                buildings_ui(
                                    ui,
                                    settlement,
                                    &mut game_state,
                                    &mut selected_building,
                                );
                                population_info(ui, settlement);
                            },
                        );

                        egui::ScrollArea::vertical().id_source("resources").show(
                            &mut columns[1],
                            |ui| {
                                resource_info(ui, settlement);
                                ui.add_space(5.);
                                production_ui(ui, settlement);
                            },
                        );
                    });
                }
            },
        );

        if !open {
            events.send(CloseSettlementUIEvent);
        }
    }
}
