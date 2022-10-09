use bevy_ecs_tilemap::prelude::*;

pub const TILES_PER_COLUMN: u32 = 8;

pub const GRASS: TileTexture = TileTexture(0);
pub const WATER: TileTexture = TileTexture(TILES_PER_COLUMN);
pub const WATER_ANIMATION_STEPS: u32 = 4;
pub const MOUNTAIN: TileTexture = TileTexture(2);
pub const HILLS: TileTexture = TileTexture(4);
pub const WOODS: TileTexture = TileTexture(6);
pub const SETTLEMENT: TileTexture = TileTexture(2 * TILES_PER_COLUMN);
pub const OUTPOST: TileTexture = TileTexture(2 * TILES_PER_COLUMN + 2);
pub const PLAYER_MARKER: TileTexture = TileTexture(2 * TILES_PER_COLUMN + 4);

pub const TILEMAP_SIZE: f32 = 16.0;
pub const TILEMAP_COLUMNS: usize = 8;
pub const TILEMAP_ROWS: usize = 4;

pub const Z_GROUND: f32 = -0.03;
pub const Z_FEATURES: f32 = -0.02;
pub const Z_MARKER: f32 = -0.01;
