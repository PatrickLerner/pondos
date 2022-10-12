use crate::{game_state::RunningState, player::PlayerTravelEvent, ui::large_button};
use bevy::prelude::*;
use bevy_egui::{egui::Frame, EguiContext};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEventAction {
    pub label: String,
    pub trigger_event: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum GameEventTrigger {
    Travel,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEvent {
    pub id: String,
    pub title: String,
    pub image: String,
    pub trigger: Option<GameEventTrigger>,
    pub text: String,
    pub actions: Vec<GameEventAction>,
}

#[derive(Default)]
pub struct CurrentGameEvents(pub HashSet<String>);

pub struct TriggerEvent {
    pub trigger: GameEventTrigger,
}

pub fn event_trigger_handler(
    mut triggers: EventReader<TriggerEvent>,
    events: Option<Res<HashMap<String, GameEvent>>>,
    mut current_events: ResMut<CurrentGameEvents>,
    mut running_state: ResMut<State<RunningState>>,
) {
    if events.is_none() {
        return;
    };
    let events = events.unwrap();
    let mut added_events = false;

    for trigger in triggers.iter() {
        for (id, event) in events.iter() {
            if event.trigger == Some(trigger.trigger) {
                current_events.0.insert(id.to_owned());
                added_events = true;
            }
        }
    }

    if added_events {
        running_state.set(RunningState::Paused).unwrap();
    }
}

pub fn event_travel(
    mut events: EventReader<PlayerTravelEvent>,
    mut triggers: EventWriter<TriggerEvent>,
) {
    for _ in events.iter() {
        triggers.send(TriggerEvent {
            trigger: GameEventTrigger::Travel,
        });
    }
}

#[derive(Default)]
pub struct EventTextures {
    #[allow(dead_code)]
    references: Vec<Handle<Image>>,
    textures: HashMap<String, bevy_egui::egui::TextureId>,
}

pub fn event_display(
    mut egui_context: ResMut<EguiContext>,
    events: Option<Res<HashMap<String, GameEvent>>>,
    mut current_events: ResMut<CurrentGameEvents>,
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

    if current_events.0.is_empty() {
        running_state.set(RunningState::Running).unwrap();
    }

    for id in current_events.0.clone().iter() {
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

        let size = (500., 300.);
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
                                current_events.0.remove(id);
                                if let Some(id) = &action.trigger_event {
                                    current_events.0.insert(id.clone());
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
                            ui.label(&event.text);
                        });
                });
            });
    }
}

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggerEvent>()
            .add_system_set(SystemSet::on_update(RunningState::Paused).with_system(event_display))
            .add_system(event_trigger_handler)
            .add_system(event_travel);
    }
}
