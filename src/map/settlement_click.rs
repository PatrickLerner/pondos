use super::CursorPos;
use crate::{
    settlement::{SelectedSettlement, Settlement},
    GameState, Player,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn settlement_click(
    cursor_pos: Res<CursorPos>,
    input_mouse: Res<Input<MouseButton>>,
    tilemap_query: Query<&TileStorage>,
    settlements: Query<&Settlement>,
    mut selected_settlement: ResMut<Option<SelectedSettlement>>,
    mut game_state: ResMut<State<GameState>>,
    player: Res<Player>,
) {
    if input_mouse.just_pressed(MouseButton::Left) {
        for tilemap in tilemap_query.iter() {
            let x = (cursor_pos.0.x / 16.0).floor() as u32;
            let y = (cursor_pos.0.y / 16.0).floor() as u32;

            if let Some(entity) = tilemap.get(&TilePos { x, y }) {
                if let Ok(settlement) = settlements.get(entity) {
                    *selected_settlement = Some(entity.into());
                    if player.location == Some(entity) {
                        log::info!("Open settlement {}", settlement.name);
                        game_state.push(GameState::Settlement).unwrap();
                    } else {
                        game_state.push(GameState::TravelToSettlement).unwrap();
                    }
                }
            }
        }
    }
}
