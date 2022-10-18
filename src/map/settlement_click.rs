use crate::{
    game_state::GameState,
    map::{constants::TILEMAP_SIZE, CursorPos},
    settlement::VisitSettlementEvent,
    types::{Player, Settlement},
    ui::SelectedSettlement,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn settlement_click(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    input_mouse: Res<Input<MouseButton>>,
    tilemap_query: Query<&TileStorage>,
    settlements: Query<&Settlement>,
    mut visit_events: EventWriter<VisitSettlementEvent>,
    resources: (Res<Player>, ResMut<State<GameState>>),
) {
    let (player, mut game_state) = resources;

    if input_mouse.just_pressed(MouseButton::Left) {
        for tilemap in tilemap_query.iter() {
            let x = (cursor_pos.0.x / TILEMAP_SIZE + 0.5).floor() as u32;
            let y = (cursor_pos.0.y / TILEMAP_SIZE + 0.5).floor() as u32;

            if let Some(entity) = tilemap.get(&TilePos { x, y }) {
                if settlements.get(entity).is_ok() {
                    commands.insert_resource(SelectedSettlement(entity));
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
