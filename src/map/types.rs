use crate::{
    map::constants::WATER_ANIMATION_STEPS,
    map::constants::{GRASS, HILLS, MOUNTAIN, OUTPOST, SETTLEMENT, WATER, WOODS},
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub enum MapTileType {
    Grass,
    Water,
    Mountain,
    Hills,
    Woods,
    Settlement,
    Outpost,
}

impl MapTileType {
    pub fn texture(&self, winter: bool) -> TileTexture {
        let base = match self {
            MapTileType::Grass => GRASS,
            MapTileType::Hills => HILLS,
            MapTileType::Mountain => MOUNTAIN,
            MapTileType::Water => WATER,
            MapTileType::Woods => WOODS,
            MapTileType::Settlement => SETTLEMENT,
            MapTileType::Outpost => OUTPOST,
        };

        if winter {
            TileTexture(base.0 + self.animation_count())
        } else {
            base
        }
    }

    pub fn animation_count(&self) -> u32 {
        if *self == MapTileType::Water {
            WATER_ANIMATION_STEPS
        } else {
            1
        }
    }

    pub fn ground(&self) -> bool {
        *self == MapTileType::Grass || *self == MapTileType::Water
    }
}
