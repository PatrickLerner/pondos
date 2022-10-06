use super::{Resource, SelectedSettlement, Settlement};
use crate::{GameState, Player};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, Align2, RichText, Ui},
    EguiContext,
};

struct TradeRow<'a> {
    ui: &'a mut Ui,
    resource: String,
    settlement: &'a mut Settlement,
    player: &'a mut Player,
    sell_price: u32,
    buy_price: u32,
}

impl<'a> TradeRow<'a> {
    fn render(&mut self) {
        self.ui.label(&self.resource);
        let player_count = *self.player.resources.get(&self.resource).unwrap_or(&0);
        let settlement_count = *self.settlement.resources.get(&self.resource).unwrap_or(&0);

        {
            self.ui
                .with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                    ui.label(format!("{}", player_count));
                });
        }

        {
            let button = self.ui.add_sized(
                [60., 20.],
                egui::Button::new(format!("sell ({})", self.sell_price)),
            );

            if button.clicked() && player_count > 0 && self.settlement.gold >= self.sell_price {
                *self
                    .settlement
                    .resources
                    .entry(self.resource.clone())
                    .or_default() += 1;
                *self
                    .player
                    .resources
                    .entry(self.resource.clone())
                    .or_default() -= 1;
                self.player.gold += self.sell_price;
                self.settlement.gold -= self.sell_price;
                log::info!("sell {} for {}", self.resource, self.sell_price);
            }
        }

        {
            let button = self.ui.add_sized(
                [60., 20.],
                egui::Button::new(format!("buy ({})", self.buy_price)),
            );

            if button.clicked() && settlement_count > 0 && self.player.gold >= self.buy_price {
                *self
                    .settlement
                    .resources
                    .entry(self.resource.clone())
                    .or_default() -= 1;
                *self
                    .player
                    .resources
                    .entry(self.resource.clone())
                    .or_default() += 1;
                self.player.gold -= self.buy_price;
                self.settlement.gold += self.buy_price;
                log::info!("buy {} for {}", self.resource, self.buy_price);
            }
        }

        {
            self.ui
                .with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                    ui.label(format!("{}", settlement_count));
                });
        }
    }
}

pub struct PriceCalculator {
    base_price: u32,
    demand: u32,
    supply: u32,
}

const MIN_SHORTAGE_MOD: f32 = 0.8;
const MAX_SHORTAGE_MOD: f32 = 2.0;

impl PriceCalculator {
    fn shortage_mod(&self) -> f32 {
        let res = (self.demand as f32 - self.supply as f32) / self.demand as f32;

        f32::clamp(1. + res, MIN_SHORTAGE_MOD, MAX_SHORTAGE_MOD)
    }

    /// price for which player can buy
    fn buy_price(&self) -> u32 {
        (self.base_price as f32 * self.shortage_mod()).ceil() as u32
    }

    /// price for which player can sell
    fn sell_price(&self) -> u32 {
        let price = Self {
            base_price: self.base_price,
            demand: self.demand,
            supply: self.supply + 1,
        };

        // we would buy for price we could sell it at if we had one more
        price.buy_price()
    }
}

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
    resources: Res<Vec<Resource>>,
    windows: Res<Windows>,
) {
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
                                ui.label(
                                    RichText::new("Resource")
                                        .text_style(crate::ui_config::panel_heading())
                                        .strong(),
                                );
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
