use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{helpers::camera::pan_orbit_camera, GameState};

// TODO:
pub mod on_enter;
mod on_exit;
mod settlement_click;
mod update_cursor_pos;
mod update_player_position;

const TILES_PER_COLUMN: u32 = 7;

const GRASS: TileTexture = TileTexture(0);
const WATER: TileTexture = TileTexture(24 * TILES_PER_COLUMN);
const MOUNTAIN: TileTexture = TileTexture(TILES_PER_COLUMN + 5);
const HILLS: TileTexture = TileTexture(TILES_PER_COLUMN + 6);
const WOODS: TileTexture = TileTexture(6);
const SETTLEMENT: TileTexture = TileTexture(2 * TILES_PER_COLUMN + 1);

const MARKER: u32 = 42 * TILES_PER_COLUMN + 6;

const TILEMAP_SIZE: f32 = 16.0;
const TILEMAP_COLUMNS: usize = 7;
const TILEMAP_ROWS: usize = 51;

const Z_GROUND: f32 = -0.03;
const Z_FEATURES: f32 = -0.02;
const Z_MARKER: f32 = -0.01;

#[derive(Clone, Copy)]
pub struct MapSize {
    pub width: u32,
    pub height: u32,
}

impl From<MapSize> for TilemapSize {
    fn from(map: MapSize) -> Self {
        Self {
            x: map.width,
            y: map.height,
        }
    }
}

#[derive(Default, Debug)]
pub struct CursorPos(Vec2);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .add_system_set(SystemSet::on_enter(GameState::LoadingMap).with_system(on_enter::setup))
            .add_system_set(
                SystemSet::on_update(GameState::LoadingMap)
                    .with_system(on_enter::transition)
                    .with_system(on_enter::load_map)
                    .with_system(on_enter::load_settlements),
            )
            .add_system_set(SystemSet::on_exit(GameState::Map).with_system(on_exit::on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::Map)
                    .with_system(pan_orbit_camera)
                    .with_system(update_cursor_pos::update_cursor_pos)
                    .with_system(settlement_click::settlement_click)
                    .with_system(update_player_position::update_player_position),
            );
    }
}
