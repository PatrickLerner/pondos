use bevy::prelude::*;
use std::collections::HashMap;

use crate::game_time::GameTimeAdvanceEvent;

#[derive(Default)]
pub struct Player {
    pub position: Vec2,
    pub location: Option<Entity>,
    pub gold: u32,
    pub resources: HashMap<String, u32>,
    pub location_marker: Option<Entity>,
    pub location_marker_texture_atlas_handle: Option<Handle<TextureAtlas>>,
    pub location_marker_need_update: bool,
}

impl Player {
    pub fn update_position(&mut self, position: Vec2, location: Option<Entity>) {
        self.position = position;
        self.location = location;
        self.location_marker_need_update = true;
    }

    pub fn resource_space_total(&self) -> u32 {
        10
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
