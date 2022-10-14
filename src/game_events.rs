use crate::{
    game_state::RunningState,
    player::PlayerTravelEvent,
    settlement::{Settlement, VisitSettlementEvent},
    ui::large_button,
};
use bevy::prelude::*;
use bevy_egui::{egui::Frame, EguiContext};
use rand::{seq::SliceRandom, thread_rng, Rng};
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
    Settlement,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEvent {
    pub id: String,
    pub title: String,
    pub image: String,
    pub trigger: Option<GameEventTrigger>,
    pub trigger_scope: Option<String>,
    pub limit: Option<u32>,
    pub chance: Option<f32>,
    pub text: String,
    pub actions: Vec<GameEventAction>,
}

#[derive(Default)]
pub struct CurrentGameEvents(pub HashSet<String>);

#[derive(Debug)]
pub struct TriggerEvent {
    pub trigger: GameEventTrigger,
    pub scope: Option<String>,
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
        let mut events: Vec<&GameEvent> = events
            .iter()
            .filter_map(|(_, event)| {
                if event.trigger == Some(trigger.trigger) && event.trigger_scope == trigger.scope {
                    Some(event)
                } else {
                    None
                }
            })
            .collect();

        events.shuffle(&mut thread_rng());

        log::debug!(
            "Trigger {:?} ({:?}) - {} events for consideration",
            trigger.trigger,
            trigger.scope,
            events.len()
        );

        let mut random = thread_rng();
        for event in events.iter() {
            if let Some(chance) = event.chance {
                if random.gen_range(0.0..1.0) > chance {
                    continue;
                }
            }

            log::info!("trigger game event {}", event.id);
            current_events.0.insert(event.id.to_owned());
            added_events = true;
            break;
        }
    }

    if added_events {
        log::debug!("pausing game due to events");
        running_state.overwrite_set(RunningState::Paused).unwrap();
    }
}

pub fn event_travel(
    mut events: EventReader<PlayerTravelEvent>,
    mut triggers: EventWriter<TriggerEvent>,
) {
    for _ in events.iter() {
        triggers.send(TriggerEvent {
            trigger: GameEventTrigger::Travel,
            scope: None,
        });
    }
}

fn event_visit_settlement(
    mut events: EventReader<VisitSettlementEvent>,
    mut triggers: EventWriter<TriggerEvent>,
    settlements: Query<&Settlement>,
) {
    for event in events.iter() {
        let settlement = settlements.get(event.settlement).unwrap();

        triggers.send(TriggerEvent {
            trigger: GameEventTrigger::Settlement,
            scope: Some(settlement.name.to_owned()),
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
        log::debug!("resuming game due to no events");
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
            .add_system(event_travel)
            .add_system(event_visit_settlement);
    }
}
