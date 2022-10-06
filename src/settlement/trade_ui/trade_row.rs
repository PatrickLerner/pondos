use crate::{settlement::Settlement, Player};
use bevy_egui::egui::{self, Align, Ui};

pub struct TradeRow<'a> {
    pub ui: &'a mut Ui,
    pub resource: String,
    pub settlement: &'a mut Settlement,
    pub player: &'a mut Player,
    pub sell_price: u32,
    pub buy_price: u32,
}

impl<'a> TradeRow<'a> {
    pub fn render(&mut self) {
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
