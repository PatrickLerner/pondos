use bevy_ecs_tilemap::prelude::*;

pub const TILES_PER_COLUMN: u32 = 7;

pub const GRASS: TileTexture = TileTexture(0);
pub const WATER: TileTexture = TileTexture(24 * TILES_PER_COLUMN);
pub const MOUNTAIN: TileTexture = TileTexture(TILES_PER_COLUMN + 5);
pub const HILLS: TileTexture = TileTexture(TILES_PER_COLUMN + 6);
pub const WOODS: TileTexture = TileTexture(6);
pub const SETTLEMENT: TileTexture = TileTexture(2 * TILES_PER_COLUMN + 1);

pub const MARKER: u32 = 42 * TILES_PER_COLUMN + 6;

pub const TILEMAP_SIZE: f32 = 16.0;
pub const TILEMAP_COLUMNS: usize = 7;
pub const TILEMAP_ROWS: usize = 51;

pub const Z_GROUND: f32 = -0.03;
pub const Z_FEATURES: f32 = -0.02;
pub const Z_MARKER: f32 = -0.01;
