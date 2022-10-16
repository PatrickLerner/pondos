use crate::{
    game_events::{GameEvent, GameEventsState},
    game_state::RunningState,
    ui::large_button,
};
use bevy::prelude::*;
use bevy_egui::{egui::Frame, EguiContext};
use std::collections::HashMap;

#[derive(Default)]
pub struct EventTextures {
    #[allow(dead_code)]
    references: Vec<Handle<Image>>,
    textures: HashMap<String, bevy_egui::egui::TextureId>,
}

pub fn event_display(
    mut egui_context: ResMut<EguiContext>,
    events: Option<Res<HashMap<String, GameEvent>>>,
    mut state: ResMut<GameEventsState>,
    mut running_state: ResMut<State<RunningState>>,
    mut textures: Local<EventTextures>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    if events.is_none() {
        return;
    };

    let events = events.unwrap();
    let window = windows.primary();

    if state.current_events.is_empty() {
        running_state.set(RunningState::Running).unwrap();
    }

    for id in state.current_events.clone().iter() {
        let event: &GameEvent = events.get(id).unwrap();

        if !textures.textures.contains_key(&event.image) {
            let image = asset_server.load(&format!("images/{}.jpg", event.image));
            textures.textures.insert(
                event.image.to_owned(),
                egui_context.add_image(image.as_weak()),
            );
            textures.references.push(image);
        }
        let image = textures.textures.get(&event.image).unwrap();

        let h = 30. * event.actions.len() as f32;
        let size = (500., 280. + h);
        bevy_egui::egui::Window::new(&event.title)
            .collapsible(false)
            .default_pos((
                (window.width() - size.0) / 2.0,
                (window.height() - size.1) / 2.0,
            ))
            .id(bevy_egui::egui::Id::new(id))
            .show(egui_context.ctx_mut(), |ui| {
                ui.set_width(size.0);
                ui.set_height(size.1);

                bevy_egui::egui::TopBottomPanel::bottom("footer")
                    .frame(Frame::none())
                    .show_inside(ui, |ui| {
                        for action in &event.actions {
                            let w = ui.available_width();
                            if large_button(ui, w, &action.label).clicked() {
                                state.current_events.remove(id);
                                if let Some(id) = &action.trigger_event {
                                    state.current_events.insert(id.clone());
                                }
                            }
                        }
                    });
                bevy_egui::egui::CentralPanel::default().show_inside(ui, |ui| {
                    let w = ui.available_width();
                    let h = 9. / 30. * w;
                    ui.add_space(5.);
                    ui.add(bevy_egui::egui::widgets::Image::new(*image, [w, h]));
                    ui.add_space(10.);
                    bevy_egui::egui::ScrollArea::vertical()
                        .id_source("content")
                        .show(ui, |ui| {
                            ui.set_width(w - 20.);

                            ui.label(&event.text);
                        });
                });
            });
    }
}
