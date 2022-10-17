use crate::game_time::GameTimeAdvanceEvent;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Ship {
    pub damage: u32,
    pub size: ShipSize,
}

#[derive(Clone, Copy, Debug)]
pub enum ShipSize {
    Small,
    Medium,
    Large,
}

impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.size {
            ShipSize::Small => write!(f, "Small Ship"),
            ShipSize::Medium => write!(f, "Medium Ship"),
            ShipSize::Large => write!(f, "Large Ship"),
        }
    }
}

impl Ship {
    pub fn new(size: ShipSize) -> Self {
        Self { size, damage: 0 }
    }

    pub fn health(&self) -> f32 {
        let max = self.max_health() as f32;

        let health = (max - self.damage as f32) / max;
        health.max(0.)
    }

    pub fn repair_price(&self) -> u32 {
        let full_repair_price = self.price() as f32 * 0.8;

        (full_repair_price * (1. - self.health())).ceil() as u32
    }

    pub fn max_health(&self) -> u32 {
        match self.size {
            ShipSize::Small => 20,
            ShipSize::Medium => 50,
            ShipSize::Large => 100,
        }
    }

    pub fn resource_space(&self) -> u32 {
        match self.size {
            ShipSize::Small => 20,
            ShipSize::Medium => 50,
            ShipSize::Large => 100,
        }
    }

    pub fn price(&self) -> u32 {
        match self.size {
            ShipSize::Small => 2000,
            ShipSize::Medium => 4000,
            ShipSize::Large => 7500,
        }
    }

    pub fn construction_time(&self) -> u32 {
        match self.size {
            ShipSize::Small => 3,
            ShipSize::Medium => 4,
            ShipSize::Large => 5,
        }
    }

    pub fn repair_time(&self) -> u32 {
        let full_repair_time = self.construction_time() as f32;

        (full_repair_time * (1. - self.health())).ceil() as u32
    }
}

const BASE_RESOURCE_SPACE: u32 = 5;

#[derive(Default, Debug)]
pub struct Player {
    pub position: Vec2,
    pub location: Option<Entity>,
    pub silver: u32,
    pub resources: HashMap<String, u32>,
    pub location_marker: Option<Entity>,
    pub location_marker_texture_atlas_handle: Option<Handle<TextureAtlas>>,
    pub location_marker_need_update: bool,
    pub convoy: Vec<Ship>,
}

impl Player {
    pub fn update_position(&mut self, position: Vec2, location: Option<Entity>) {
        self.position = position;
        self.location = location;
        self.location_marker_need_update = true;
    }

    pub fn resource_space_total(&self) -> u32 {
        BASE_RESOURCE_SPACE
            + self
                .convoy
                .iter()
                .fold(0, |acc, transport| acc + transport.resource_space())
    }

    pub fn resource_space_left(&self) -> u32 {
        self.resource_space_total() - self.resource_space_used()
    }

    pub fn resource_space_used(&self) -> u32 {
        self.resources.iter().fold(0, |acc, (_, count)| acc + count)
    }
}

pub struct PlayerTravelEvent {
    position: Vec2,
    entity: Entity,
}

impl PlayerTravelEvent {
    pub fn new(entity: Entity, x: u32, y: u32) -> Self {
        let position = Vec2::new(x as f32, y as f32);

        Self { position, entity }
    }
}

pub struct PlayerShipwreckEvent {
    pub ship_index: usize,
}

pub fn handle_travel(
    mut events: EventReader<PlayerTravelEvent>,
    mut player: Option<ResMut<Player>>,
    mut advance_time_events: EventWriter<GameTimeAdvanceEvent>,
) {
    for event in events.iter() {
        if let Some(player) = &mut player {
            if player.position != event.position {
                log::info!(
                    "Player traveled to {}:{}",
                    event.position.x,
                    event.position.y
                );
                player.update_position(event.position, Some(event.entity));
                advance_time_events.send(GameTimeAdvanceEvent);
            }
        }
    }
}

pub fn shipwreck_check(
    player: Option<Res<Player>>,
    mut shipwreck_events: EventWriter<PlayerShipwreckEvent>,
) {
    if let Some(player) = player {
        for (ship_index, ship) in player.convoy.iter().enumerate() {
            if ship.health() == 0. {
                shipwreck_events.send(PlayerShipwreckEvent { ship_index });
            }
        }
    }
}

pub fn shipwreck_remove(
    player: Option<ResMut<Player>>,
    mut shipwreck_events: EventReader<PlayerShipwreckEvent>,
) {
    if shipwreck_events.is_empty() {
        return;
    }

    if let Some(mut player) = player {
        let remove_index: Vec<usize> = shipwreck_events
            .iter()
            .map(|event| event.ship_index)
            .collect();

        log::info!("{} ship(s) shipwrecked", remove_index.len());

        player.convoy = player
            .convoy
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(index, ship)| {
                if remove_index.contains(&index) {
                    None
                } else {
                    Some(ship)
                }
            })
            .collect();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerTravelEvent>()
            .add_event::<PlayerShipwreckEvent>()
            .add_system(shipwreck_check)
            .add_system(shipwreck_remove)
            .add_system(handle_travel);
    }
}
