use crate::{
    game_events::{AddEventToCurrentEvent, GameEvent, GameEventsState},
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
    mut textures: Local<EventTextures>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    mut add_event: EventWriter<AddEventToCurrentEvent>,
) {
    if events.is_none() {
        return;
    };

    let events = events.unwrap();
    let window = windows.primary();

    if let Some(id) = state.current_events.first() {
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
                                state.current_events.remove(0);
                                if let Some(id) = &action.trigger_event.clone() {
                                    add_event
                                        .send(AddEventToCurrentEvent::new_to_front(id.clone()));
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
