use super::{Population, SelectedSettlement, Settlement};
use crate::{GameState, Player};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText, Ui},
    EguiContext,
};

struct TradeRow<'a> {
    ui: &'a mut Ui,
    name: &'a str,
    player_count: &'a mut u32,
    settlement_count: &'a mut u32,
    player_gold: &'a mut u32,
    settlement_gold: &'a mut u32,
    sell_price: Option<u32>,
    buy_price: Option<u32>,
}

impl<'a> TradeRow<'a> {
    fn render(&mut self) {
        self.ui.label(self.name);
        self.ui
            .with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                ui.label(format!("{}", self.player_count));
            });

        if let Some(price) = self.sell_price {
            if self.ui.small_button(format!("sell ({})", price)).clicked()
                && *self.player_count > 0
                && *self.settlement_gold >= price
            {
                *self.settlement_count += 1;
                *self.player_count -= 1;
                *self.player_gold += price;
                *self.settlement_gold -= price;
                log::info!("sell {} for {}", self.name, price);
            }
        } else {
            self.ui.label("");
        }
        self.ui
            .with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                ui.label(format!("{}", self.settlement_count));
            });
        if let Some(price) = self.buy_price {
            if self.ui.small_button(format!("buy ({})", price)).clicked()
                && *self.settlement_count > 0
                && *self.player_gold >= price
            {
                *self.settlement_count -= 1;
                *self.player_count += 1;
                *self.player_gold -= price;
                *self.settlement_gold += price;
                log::info!("buy {} for {}", self.name, price);
            }
        } else {
            self.ui.label("");
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

fn demand_food(pop_count: u32) -> u32 {
    (1.5 * pop_count as f32).ceil() as u32
}

fn demand_livestock(farmer_count: u32) -> u32 {
    6 * farmer_count
}

const BASE_PRICE_GRAIN: u32 = 3;
const BASE_PRICE_DAIRY: u32 = 8;
const BASE_PRICE_MEAT: u32 = 15;
const BASE_PRICE_FISH: u32 = 10;
const BASE_PRICE_LIVESTOCK: u32 = 25;

pub fn trade_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_settlement: Res<Option<SelectedSettlement>>,
    mut settlements: Query<&mut Settlement>,
    mut player: ResMut<Player>,
    mut game_state: ResMut<State<GameState>>,
) {
    if let Some(entity) = selected_settlement.as_ref() {
        let mut settlement = settlements
            .get_mut(entity.0)
            .expect("Expected settlement to be selected");

        egui::TopBottomPanel::bottom("footer").show(egui_context.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                let button = ui.add_sized([120., 40.], egui::Button::new("Close"));

                if button.clicked() {
                    game_state.pop().unwrap();
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

            egui::Grid::new("resources").show(ui, |ui| {
                ui.set_width(500.);

                let pop_count = settlement.populations.len() as u32;
                let farmer_count = settlement
                    .populations
                    .clone()
                    .into_iter()
                    .filter(|p| *p == Population::Farmer)
                    .count() as u32;

                let player = &mut player.resources;
                let settlement = &mut settlement.resources;

                TradeRow {
                    ui,
                    name: "Gold",
                    player_count: &mut player.gold,
                    settlement_count: &mut settlement.gold,
                    player_gold: &mut 0,
                    settlement_gold: &mut 0,
                    sell_price: None,
                    buy_price: None,
                }
                .render();
                ui.end_row();

                let prices = PriceCalculator {
                    base_price: BASE_PRICE_GRAIN,
                    demand: demand_food(pop_count),
                    supply: settlement.grain,
                };

                TradeRow {
                    ui,
                    name: "Grain",
                    player_count: &mut player.grain,
                    settlement_count: &mut settlement.grain,
                    player_gold: &mut player.gold,
                    settlement_gold: &mut settlement.gold,
                    sell_price: Some(prices.sell_price()),
                    buy_price: Some(prices.buy_price()),
                }
                .render();
                ui.end_row();

                let prices = PriceCalculator {
                    base_price: BASE_PRICE_DAIRY,
                    demand: demand_food(pop_count),
                    supply: settlement.dairy,
                };

                TradeRow {
                    ui,
                    name: "Dairy",
                    player_count: &mut player.dairy,
                    settlement_count: &mut settlement.dairy,
                    player_gold: &mut player.gold,
                    settlement_gold: &mut settlement.gold,
                    sell_price: Some(prices.sell_price()),
                    buy_price: Some(prices.buy_price()),
                }
                .render();
                ui.end_row();

                let prices = PriceCalculator {
                    base_price: BASE_PRICE_MEAT,
                    demand: demand_food(pop_count),
                    supply: settlement.meat,
                };

                TradeRow {
                    ui,
                    name: "Meat",
                    player_count: &mut player.meat,
                    settlement_count: &mut settlement.meat,
                    player_gold: &mut player.gold,
                    settlement_gold: &mut settlement.gold,
                    sell_price: Some(prices.sell_price()),
                    buy_price: Some(prices.buy_price()),
                }
                .render();
                ui.end_row();

                let prices = PriceCalculator {
                    base_price: BASE_PRICE_FISH,
                    demand: demand_food(pop_count),
                    supply: settlement.fish,
                };

                TradeRow {
                    ui,
                    name: "Fish",
                    player_count: &mut player.fish,
                    settlement_count: &mut settlement.fish,
                    player_gold: &mut player.gold,
                    settlement_gold: &mut settlement.gold,
                    sell_price: Some(prices.sell_price()),
                    buy_price: Some(prices.buy_price()),
                }
                .render();
                ui.end_row();

                let prices = PriceCalculator {
                    base_price: BASE_PRICE_LIVESTOCK,
                    demand: demand_livestock(farmer_count),
                    supply: settlement.livestock,
                };

                TradeRow {
                    ui,
                    name: "Livestock",
                    player_count: &mut player.livestock,
                    settlement_count: &mut settlement.livestock,
                    player_gold: &mut player.gold,
                    settlement_gold: &mut settlement.gold,
                    sell_price: Some(prices.sell_price()),
                    buy_price: Some(prices.buy_price()),
                }
                .render();
                ui.end_row();
            });
        });
    }
}
