use super::{ResourceType, Resources, SelectedSettlement, Settlement};
use crate::{GameState, Player};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText, Ui},
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

        self.ui
            .with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                ui.label(format!("{}", player_count));
            });

        if self
            .ui
            .small_button(format!("sell ({})", self.sell_price))
            .clicked()
            && player_count > 0
            && self.settlement.gold >= self.sell_price
        {
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

        self.ui
            .with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                ui.label(format!("{}", settlement_count));
            });

        if self
            .ui
            .small_button(format!("buy ({})", self.buy_price))
            .clicked()
            && settlement_count > 0
            && self.player.gold >= self.buy_price
        {
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

pub fn trade_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_settlement: Res<Option<SelectedSettlement>>,
    mut settlements: Query<&mut Settlement>,
    mut player: ResMut<Player>,
    mut game_state: ResMut<State<GameState>>,
    resources: Res<Resources>,
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
                    // TODO:
                    .filter(|p| *p == "Farmer")
                    .count() as u32;

                {
                    ui.label("Gold");
                    ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                        ui.label(format!("{}", player.gold));
                    });
                    ui.label("");
                    ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                        ui.label(format!("{}", settlement.gold));
                    });
                    ui.label("");
                    ui.end_row();
                }

                // TODO: demand_food more dynamic
                for resource in resources.0.iter() {
                    let demand = match resource.resource_type {
                        ResourceType::Food => demand_food(pop_count),
                        ResourceType::Livestock => demand_livestock(farmer_count),
                    };

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
    }
}
