use super::CloseSettlementUIEvent;
use bevy::prelude::*;

pub fn close_by_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<CloseSettlementUIEvent>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        events.send(CloseSettlementUIEvent);
    }
}
