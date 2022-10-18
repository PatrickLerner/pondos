use crate::types::Ship;
use bevy::prelude::*;
use std::collections::HashMap;

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
