use crate::{settlement::Settlement, Player};
use bevy::prelude::*;

pub fn load_player(mut commands: Commands, settlements: Query<(Entity, &Settlement)>) {
    if let Some((entity, settlement)) = settlements.iter().last() {
        let player = Player {
            gold: 350,
            position: Vec2::new(settlement.position.x as f32, settlement.position.y as f32),
            location: Some(entity),
            location_marker_need_update: true,
            ..default()
        };

        commands.insert_resource(player);
    }
}
