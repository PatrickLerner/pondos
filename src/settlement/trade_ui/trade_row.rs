use crate::{player::Player, settlement::Settlement};
use bevy_egui::egui::{self, Align, Color32, RichText, Ui};

pub struct TradeRow<'a> {
    pub ui: &'a mut Ui,
    pub resource: String,
    pub settlement: &'a mut Settlement,
    pub player: &'a mut Player,
    pub sell_price: u32,
    pub buy_price: u32,
    pub average_price: f32,
}

fn enabled_color(enabled: bool) -> Color32 {
    if enabled {
        Color32::BLACK
    } else {
        Color32::GRAY
    }
}

fn button(ui: &mut Ui, text: String, enabled: bool) -> egui::Response {
    ui.add_sized(
        [60., 20.],
        egui::Button::new(RichText::new(text).color(enabled_color(enabled))),
    )
}

impl<'a> TradeRow<'a> {
    pub fn render(&mut self) {
        let player_count = *self.player.resources.get(&self.resource).unwrap_or(&0);
        let settlement_count = *self.settlement.resources.get(&self.resource).unwrap_or(&0);

        self.ui.label(
            RichText::new(&self.resource)
                .color(enabled_color(player_count > 0 || settlement_count > 0)),
        );

        {
            self.ui
                .with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                    ui.label(format!("{}", player_count));
                });
        }

        {
            let enabled = player_count > 0 && self.settlement.gold >= self.sell_price;
            let text = format!("sell ({})", self.sell_price);

            if button(self.ui, text, enabled).clicked() && enabled {
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
            let enabled = settlement_count > 0 && self.player.gold >= self.buy_price;
            let text = format!("buy ({})", self.buy_price);

            if button(self.ui, text, enabled).clicked() && enabled {
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

        {
            let rel = (((self.sell_price as f32 - self.average_price) / self.average_price) * 100.)
                .round() as i8;

            let wording = match rel {
                -100..=-50 => "extremely cheap",
                -49..=-25 => "cheap",
                -24..=-10 => "below average",
                -9..=10 => "average",
                11..=25 => "above average",
                26..=50 => "expensive",
                51..=100 => "very expensive",
                _ => "unknown",
            };

            self.ui
                .with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                    ui.label(wording);
                });
        }
    }
}
