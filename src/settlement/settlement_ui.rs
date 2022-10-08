use super::{
    CloseSettlementUIEvent, SelectedSettlement, Settlement, MAX_HEIGHT, MAX_WIDTH,
    WINDOW_PADDING_X, WINDOW_PADDING_Y,
};
use crate::game_state::GameState;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, Align2, Ui},
    EguiContext,
};

pub fn settlement_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_settlement: Res<Option<SelectedSettlement>>,
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

        let window = windows.get_primary().unwrap();
        let win_max_width = window.width() - WINDOW_PADDING_X;
        let width = f32::min(win_max_width, MAX_WIDTH);
        let win_max_height = window.height() - WINDOW_PADDING_Y;
        let height = f32::min(win_max_height, MAX_HEIGHT);

        egui::Window::new(&settlement.name)
            .anchor(Align2::CENTER_CENTER, (0., 0.))
            .resizable(false)
            .open(&mut open)
            .collapsible(false)
            .show(egui_context.ctx_mut(), |ui| {
                ui.set_height(height);
                ui.set_width(width);

                ui.add_space(10.);
                ui.with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                    let button = ui.add_sized([100., 30.], egui::Button::new("Trade"));
                    if button.clicked() {
                        game_state.push(GameState::TradeWithSettlement).unwrap();
                    }
                });

                if win_max_width > 400. {
                    ui.columns(2, |columns| {
                        egui::ScrollArea::vertical()
                            .id_source("population")
                            .show(&mut columns[0], |ui| population_info(ui, settlement));

                        egui::ScrollArea::vertical()
                            .id_source("resources")
                            .show(&mut columns[1], |ui| resource_info(ui, settlement));
                    });
                } else {
                    ui.with_layout(egui::Layout::top_down_justified(Align::Min), |ui| {
                        egui::ScrollArea::both().id_source("info").show(ui, |ui| {
                            population_info(ui, settlement);
                            resource_info(ui, settlement);
                        });
                    });
                }
            });

        if !open {
            events.send(CloseSettlementUIEvent);
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

    ui.label(format!("Gold: {}", settlement.gold));
}
