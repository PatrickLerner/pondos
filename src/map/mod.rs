use crate::{
    camera::pan_orbit_camera,
    game_state::{GameState, LoadingState},
    game_time::GameTimeAdvancedEvent,
    map::{constants::TILEMAP_SIZE, types::MapTileType},
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use iyes_loopless::prelude::ConditionSet;

mod on_exit;
mod settlement_click;
mod update_cursor_pos;
mod update_player_position;

pub mod constants;
pub mod types;

#[derive(Clone, Copy)]
pub struct MapSize {
    pub width: u32,
    pub height: u32,
}

impl MapSize {
    pub fn pixel_size(&self) -> Vec2 {
        Vec2::new(
            self.width as f32 * TILEMAP_SIZE,
            self.height as f32 * TILEMAP_SIZE,
        )
    }
}

impl From<MapSize> for TilemapSize {
    fn from(map: MapSize) -> Self {
        Self {
            x: map.width,
            y: map.height,
        }
    }
}

const WINTER_SEASON: i8 = 5;
const SUMMER_SEASON: i8 = 2;

fn switch_tiles(
    tiles: &mut Query<(&mut TileTexture, Option<&mut AnimatedTile>, &MapTileType)>,
    winter: bool,
) {
    for (mut texture, animated_tile, map_tile_type) in tiles.iter_mut() {
        *texture = map_tile_type.texture(winter);

        if let Some(mut animated_tile) = animated_tile {
            animated_tile.start = texture.0;
            animated_tile.end = texture.0 + map_tile_type.animation_count();
        }
    }
}

pub fn update_tiles(
    mut events: EventReader<GameTimeAdvancedEvent>,
    mut tiles: Query<(&mut TileTexture, Option<&mut AnimatedTile>, &MapTileType)>,
) {
    for event in events.iter() {
        if event.time.is_initialized() {
            if event.time.season == WINTER_SEASON {
                log::debug!("switching to winter tiles");
                switch_tiles(&mut tiles, true)
            }
            if event.time.season == SUMMER_SEASON {
                log::debug!("switching to summer tiles");
                switch_tiles(&mut tiles, false)
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct CursorPos(Vec2);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .add_system(update_player_position::update_player_position)
            .add_system(update_cursor_pos::update_cursor_pos)
            .add_system(update_tiles)
            .add_system_set(SystemSet::on_exit(GameState::Map).with_system(on_exit::on_exit))
            .add_system_set(
                ConditionSet::new()
                    .run_in_bevy_state(GameState::Map)
                    .run_in_bevy_state(LoadingState::Loaded)
                    .with_system(pan_orbit_camera)
                    .with_system(settlement_click::settlement_click)
                    .into(),
            );
    }
}
