use crate::GameState;

use super::{CloseSettlementUIEvent, SelectedSettlement, Settlement};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText},
    EguiContext,
};

pub fn settlement_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_settlement: Res<Option<SelectedSettlement>>,
    settlements: Query<&Settlement>,
    mut events: EventWriter<CloseSettlementUIEvent>,
    mut game_state: ResMut<State<GameState>>,
) {
    if let Some(entity) = selected_settlement.as_ref() {
        let settlement = settlements
            .get(entity.0)
            .expect("Expected settlement to be selected");

        egui::TopBottomPanel::bottom("footer").show(egui_context.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                let button = ui.add_sized([120., 40.], egui::Button::new("Close"));

                if button.clicked() {
                    events.send(CloseSettlementUIEvent);
                }
            })
        });
        egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
            ui.label(
                RichText::new(&settlement.name)
                    .text_style(crate::panel_heading())
                    .strong(),
            );
            ui.add_space(15.);

            ui.columns(2, |columns| {
                egui::ScrollArea::vertical()
                    .id_source("population")
                    .show(&mut columns[0], |ui| {
                        ui.heading(format!("Population ({})", settlement.populations.len()));
                        ui.add_space(5.);

                        for population in settlement.populations.iter() {
                            ui.label(format!("{:?}", population));
                        }
                    });

                egui::ScrollArea::vertical()
                    .id_source("resources")
                    .show(&mut columns[1], |ui| {
                        ui.heading("Resources");
                        ui.add_space(5.);

                        let resources = &settlement.resources;
                        ui.label(format!("Gold: {}", settlement.gold));

                        for (resource, amount) in resources.iter() {
                            ui.label(format!("{}: {}", resource, amount));
                        }

                        ui.horizontal(|ui| {
                            if ui.button("Trade").clicked() {
                                game_state.push(GameState::TradeWithSettlement).unwrap();
                            }
                        });
                    });
            });
        });
    }
}
