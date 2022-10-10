use crate::{
    map::constants::WATER_ANIMATION_STEPS,
    map::constants::{
        GRASS, GRASS_OVERLAY_CORNER, GRASS_OVERLAY_STRAIGHT, GRASS_OVERLAY_TUNNEL, HILLS, MOUNTAIN,
        OUTPOST, SETTLEMENT, WATER, WOODS,
    },
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::constants::GRASS_OVERLAY_TUNNEL_END;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Overlay {
    pub right: bool,
    pub left: bool,
    pub top: bool,
    pub bottom: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub enum MapTileType {
    Grass,
    Water,
    Mountain,
    Hills,
    Woods,
    Settlement,
    Outpost,
    GrassOverlay(Overlay),
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
            MapTileType::GrassOverlay(overlay) => {
                let count = vec![overlay.right, overlay.left, overlay.top, overlay.bottom]
                    .iter()
                    .fold(0, |acc, item| acc + (if *item { 1 } else { 0 }));
                if count == 2 {
                    if (overlay.top && overlay.bottom) || (overlay.left && overlay.right) {
                        GRASS_OVERLAY_TUNNEL
                    } else {
                        GRASS_OVERLAY_CORNER
                    }
                } else if count == 3 {
                    GRASS_OVERLAY_STRAIGHT
                } else {
                    GRASS_OVERLAY_TUNNEL_END
                }
            }
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

    pub fn flip(&self) -> Option<TileFlip> {
        if let MapTileType::GrassOverlay(overlay) = self {
            let (x, y, d) = match (overlay.top, overlay.right, overlay.bottom, overlay.left) {
                // corners
                (true, true, false, false) => (false, true, false),
                (false, false, true, true) => (true, false, false),
                (true, false, false, true) => (true, true, false),
                // straights
                (true, true, true, false) => (false, false, true),
                (true, true, false, true) => (false, true, false),
                (true, false, true, true) => (true, false, true),
                // tunnel
                (true, false, true, false) => (false, false, true),
                // tunnel end
                (true, false, false, false) => (false, true, false),
                (false, true, false, false) => (false, false, true),
                (false, false, false, true) => (true, false, true),
                _ => (false, false, false),
            };

            return Some(TileFlip { x, y, d });
        }

        None
    }

    pub fn ground_tile(&self) -> MapTileType {
        if let MapTileType::GrassOverlay(_) = self {
            MapTileType::Water
        } else {
            MapTileType::Grass
        }
    }

    pub fn ground(&self) -> bool {
        *self == MapTileType::Grass || *self == MapTileType::Water
    }
}
