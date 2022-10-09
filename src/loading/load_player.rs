use crate::{
    player::{Player, TransportType},
    settlement::Settlement,
};
use bevy::prelude::*;

pub fn load_player(
    mut commands: Commands,
    settlements: Query<(Entity, &Settlement)>,

    player: Option<ResMut<Player>>,
) {
    if player.is_some() {
        return;
    }

    if let Some((entity, settlement)) = settlements.iter().last() {
        let player = Player {
            silver: 350,
            position: Vec2::new(settlement.position.x as f32, settlement.position.y as f32),
            location: Some(entity),
            location_marker_need_update: true,
            convoy: vec![TransportType::SmallShip],
            ..default()
        };

        log::debug!("spawn player");
        commands.insert_resource(player);
    }
}
