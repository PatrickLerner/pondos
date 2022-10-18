use crate::{
    settlement::Settlement,
    types::{Player, Ship, ShipSize},
    Settings,
};
use bevy::prelude::*;

pub fn load_player(
    mut commands: Commands,
    settlements: Query<(Entity, &Settlement)>,
    settings: Option<Res<Settings>>,
    player: Option<ResMut<Player>>,
) {
    if player.is_some() {
        return;
    }

    if let Some(settings) = settings {
        if let Some((entity, settlement)) = settlements
            .iter()
            .find(|(_, settlement)| settlement.name == settings.start_settlement)
        {
            let player = Player {
                silver: settings.start_silver,
                position: Vec2::new(settlement.position.x as f32, settlement.position.y as f32),
                location: Some(entity),
                location_marker_need_update: true,
                convoy: vec![Ship::new(ShipSize::Small)],
                ..default()
            };

            log::debug!("spawn player");
            commands.insert_resource(player);
        }
    }
}
