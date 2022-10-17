use crate::game_state::RunningState;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashSet;

mod add_event_to_current_event;
mod event_display;
mod event_effect_handler;
mod event_shipwreck;
mod event_travel;
mod event_trigger_handler;
mod event_visit_settlement;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEventAction {
    pub label: String,
    pub trigger_event: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum GameEventTriggerEventName {
    Travel,
    Settlement,
    Shipwreck,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GameEventTriggerCondition {
    pub event: GameEventTriggerEventName,
    pub scope: Option<String>,
    #[serde(default)]
    pub once: bool,
    pub chance: Option<f32>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DamageEffect {
    pub amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameEventEffect {
    DamageAnyShip(DamageEffect),
    DamageAllShips(DamageEffect),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEvent {
    pub id: String,
    pub title: String,
    pub image: String,
    pub trigger: Option<GameEventTriggerCondition>,
    #[serde(default)]
    pub effects: Vec<GameEventEffect>,
    pub text: String,
    pub actions: Vec<GameEventAction>,
}

#[derive(Default)]
pub struct GameEventsState {
    pub current_events: Vec<String>,
    pub seen_events: HashSet<String>,
}

#[derive(Debug)]
pub struct TriggerEvent {
    pub event: GameEventTriggerEventName,
    pub scope: Option<String>,
}

#[derive(Debug)]
pub struct TriggerEventEffect {
    pub effect: GameEventEffect,
}

#[derive(Debug, Copy, Clone)]
pub enum AddEventToCurrentEventPosition {
    Front,
    End,
}

#[derive(Debug)]
pub struct AddEventToCurrentEvent {
    pub id: String,
    pub position: AddEventToCurrentEventPosition,
}

impl AddEventToCurrentEvent {
    pub fn new(id: String) -> Self {
        Self {
            id,
            position: AddEventToCurrentEventPosition::End,
        }
    }

    pub fn new_to_front(id: String) -> Self {
        Self {
            id,
            position: AddEventToCurrentEventPosition::Front,
        }
    }
}

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameEventsState>()
            .add_event::<TriggerEvent>()
            .add_event::<TriggerEventEffect>()
            .add_event::<AddEventToCurrentEvent>()
            .add_system_set(
                SystemSet::on_update(RunningState::Paused)
                    .with_system(event_display::event_display),
            )
            .add_system(event_trigger_handler::event_trigger_handler)
            .add_system(add_event_to_current_event::add_event_to_current_event)
            .add_system(event_effect_handler::event_effect_handler)
            .add_system(event_travel::event_travel)
            .add_system(event_visit_settlement::event_visit_settlement)
            .add_system(event_shipwreck::event_shipwreck);
    }
}
