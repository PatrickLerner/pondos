use super::{SelectedSettlement, Settlement};
use crate::{
    create_window,
    game_state::{GameState, SettlementState},
    player::Player,
    price_calculator::{AveragePrices, PriceCalculator},
    resources::Resource,
    ui_config::large_button,
    COIN_NAME,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText},
    EguiContext,
};

mod trade_row;

use trade_row::TradeRow;

pub fn trade_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_settlement: Res<Option<SelectedSettlement>>,
    mut settlements: Query<&mut Settlement>,
    mut player: ResMut<Player>,
    mut game_state: ResMut<State<GameState>>,
    windows: Res<Windows>,
    trade_info: (Res<Vec<Resource>>, Res<AveragePrices>),
) {
    let (resources, average_prices) = trade_info;

    if let Some(entity) = selected_settlement.as_ref() {
        let mut settlement = settlements
            .get_mut(entity.0)
            .expect("Expected settlement to be selected");

        let mut open = true;

        create_window(
            egui_context.ctx_mut(),
            &windows,
            &format!("Trade with {}", settlement.name),
            &mut open,
            |ui| {
                ui.add_space(10.);
                ui.with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                    if large_button(ui, 100., "Back to Overview").clicked() {
                        game_state
                            .set(GameState::Settlement(SettlementState::Overview))
                            .unwrap()
                    }
                });

                egui::ScrollArea::both()
                    .id_source("resources")
                    .show(ui, |ui| {
                        egui::Grid::new("resources").show(ui, |ui| {
                            {
                                ui.label("");
                                ui.label(
                                    RichText::new("Convoy")
                                        .text_style(crate::ui_config::panel_heading())
                                        .strong(),
                                );
                                ui.label("");
                                ui.label("");
                                ui.label(
                                    RichText::new(&settlement.name)
                                        .text_style(crate::ui_config::panel_heading())
                                        .strong(),
                                );
                                ui.label("");
                                ui.end_row();
                            }

                            for resource in resources.iter() {
                                let demand =
                                    resource.demand.value(&settlement.populations).ceil() as u32;

                                let prices = PriceCalculator {
                                    base_price: resource.base_price,
                                    demand,
                                    supply: *settlement.resources.get(&resource.name).unwrap_or(&0),
                                };

                                TradeRow {
                                    ui,
                                    resource: resource.name.clone(),
                                    player: &mut player,
                                    settlement: &mut settlement,
                                    sell_price: prices.sell_price(),
                                    buy_price: prices.buy_price(),
                                    average_price: *average_prices
                                        .prices
                                        .get(&resource.name)
                                        .unwrap_or(&0.0),
                                }
                                .render();
                                ui.end_row();
                            }

                            {
                                for _ in 0..6 {
                                    ui.separator();
                                }
                                ui.end_row();
                            }

                            {
                                ui.label(COIN_NAME);
                                ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                                    ui.label(format!("{}", player.silver));
                                });
                                ui.label("");
                                ui.label("");
                                ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                                    ui.label(format!("{}", settlement.silver));
                                });
                                ui.label("");
                                ui.end_row();
                            }

                            {
                                ui.label("Space");
                                ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                                    ui.label(format!(
                                        "{} / {}",
                                        player.resource_space_used(),
                                        player.resource_space_total()
                                    ));
                                });
                                ui.label("");
                                ui.label("");
                                ui.label("");
                                ui.label("");
                                ui.end_row();
                            }
                        });
                    });
            },
        );

        if !open {
            game_state.pop().unwrap();
        }
    }
}
