use super::{Resource, SelectedSettlement, Settlement};
use crate::{price_calculator::PriceCalculator, AveragePrices, GameState, Player};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, Align2, RichText},
    EguiContext,
};

mod trade_row;

use trade_row::TradeRow;

const WINDOW_PADDING_X: f32 = 40.;
const WINDOW_PADDING_Y: f32 = 80.;
const MAX_WIDTH: f32 = 800.;
const MAX_HEIGHT: f32 = 600.;

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

        let window = windows.get_primary().unwrap();
        let win_max_width = window.width() - WINDOW_PADDING_X;
        let width = f32::min(win_max_width, MAX_WIDTH);
        let win_max_height = window.height() - WINDOW_PADDING_Y;
        let height = f32::min(win_max_height, MAX_HEIGHT);

        egui::Window::new(format!("Trade with {}", settlement.name))
            .anchor(Align2::CENTER_CENTER, (0., 0.))
            .resizable(false)
            .open(&mut open)
            .collapsible(false)
            .show(egui_context.ctx_mut(), |ui| {
                ui.set_height(height);
                ui.set_width(width);

                ui.add_space(10.);
                ui.with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                    let button = ui.add_sized([100., 30.], egui::Button::new("Back to Overview"));
                    if button.clicked() {
                        game_state.pop().unwrap();
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

                            {
                                ui.label("Gold");
                                ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                                    ui.label(format!("{}", player.gold));
                                });
                                ui.label("");
                                ui.label("");
                                ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                                    ui.label(format!("{}", settlement.gold));
                                });
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
                        });
                    });
            });

        if !open {
            game_state.pop().unwrap();
        }
    }
}
