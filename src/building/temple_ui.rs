use crate::{
    building::Temple,
    game_state::{GameState, SettlementState},
    player::Player,
    ui::{create_window, enabled_color, large_button, SelectedBuilding},
    COIN_NAME,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText},
    EguiContext,
};

pub fn temple_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_building: Res<Option<SelectedBuilding>>,
    mut temples: Query<&mut Temple>,
    mut game_state: ResMut<State<GameState>>,
    windows: Res<Windows>,
    mut player: ResMut<Player>,
) {
    if let Some(entity) = selected_building.as_ref() {
        if let Ok(mut temple) = temples.get_mut(entity.0) {
            let mut open = true;

            create_window(
                egui_context.ctx_mut(),
                &windows,
                &format!("Temple of {}", temple.info.deity),
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

                    let width = ui.available_width();

                    {
                        if temple.offers_made > 0 {
                            ui.add_space(5.);
                            ui.label(format!("Offers made: {} {}", temple.offers_made, COIN_NAME));
                            ui.add_space(5.);
                        }

                        for (name, price) in
                            vec![("small", 100), ("medium", 500), ("large", 1000)].into_iter()
                        {
                            let enabled = player.silver >= price;
                            let button = ui.add_sized(
                                [width, 30.],
                                egui::Button::new(
                                    RichText::new(format!(
                                        "Conduct a {} offering ({} {})",
                                        name, price, COIN_NAME
                                    ))
                                    .color(enabled_color(enabled)),
                                ),
                            );

                            if button.clicked() && enabled {
                                player.silver -= price;
                                temple.offers_made += price;
                            }
                        }
                    }

                    ui.add_space(10.);

                    {
                        if temple.temple_donations_made > 0 {
                            ui.add_space(5.);
                            ui.label(format!(
                                "Donations to temple made: {} {}",
                                temple.temple_donations_made, COIN_NAME
                            ));
                            ui.add_space(5.);
                        }

                        for (name, price) in
                            vec![("small", 100), ("medium", 500), ("large", 1000)].into_iter()
                        {
                            let enabled = player.silver >= price;
                            let button = ui.add_sized(
                                [width, 30.],
                                egui::Button::new(
                                    RichText::new(format!(
                                        "Donate a {} sum to the temple ({} {})",
                                        name, price, COIN_NAME
                                    ))
                                    .color(enabled_color(enabled)),
                                ),
                            );

                            if button.clicked() && enabled {
                                player.silver -= price;
                                temple.temple_donations_made += price;
                            }
                        }
                    }

                    ui.add_space(10.);

                    {
                        if temple.poor_donations_made > 0 {
                            ui.add_space(5.);
                            ui.label(format!(
                                "Donations to local poor people made: {} {}",
                                temple.poor_donations_made, COIN_NAME
                            ));
                            ui.add_space(5.);
                        }

                        for (name, price) in
                            vec![("small", 100), ("medium", 500), ("large", 1000)].into_iter()
                        {
                            let enabled = player.silver >= price;
                            let button = ui.add_sized(
                                [width, 30.],
                                egui::Button::new(
                                    RichText::new(format!(
                                        "Donate a {} sum to local poor people ({} {})",
                                        name, price, COIN_NAME
                                    ))
                                    .color(enabled_color(enabled)),
                                ),
                            );

                            if button.clicked() && enabled {
                                player.silver -= price;
                                temple.poor_donations_made += price;
                            }
                        }
                    }
                },
            );

            if !open {
                game_state.pop().unwrap();
            }
        }
    }
}
