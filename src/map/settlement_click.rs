use super::{constants::TILEMAP_SIZE, CursorPos};
use crate::{
    game_state::GameState,
    player::Player,
    settlement::{Settlement, VisitSettlementEvent},
    ui::SelectedSettlement,
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
    mut visit_events: EventWriter<VisitSettlementEvent>,
) {
    if input_mouse.just_pressed(MouseButton::Left) {
        for tilemap in tilemap_query.iter() {
            let x = (cursor_pos.0.x / TILEMAP_SIZE + 0.5).floor() as u32;
            let y = (cursor_pos.0.y / TILEMAP_SIZE + 0.5).floor() as u32;

            if let Some(entity) = tilemap.get(&TilePos { x, y }) {
                if settlements.get(entity).is_ok() {
                    *selected_settlement = Some(entity.into());
                    if player.location == Some(entity) {
                        visit_events.send(VisitSettlementEvent { settlement: entity })
                    } else {
                        game_state.push(GameState::Travel).unwrap();
                    }
                }
            }
        }
    }
}
