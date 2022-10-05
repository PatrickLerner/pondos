use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{camera::pan_orbit_camera, GameState};

mod on_exit;
mod settlement_click;
mod update_cursor_pos;
mod update_player_position;

pub mod constants;

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
