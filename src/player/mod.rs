use bevy::prelude::*;

mod handle_travel;
mod shipwreck_check;
mod shipwreck_remove;

pub struct PlayerTravelEvent {
    position: Vec2,
    entity: Entity,
}

impl PlayerTravelEvent {
    pub fn new(entity: Entity, x: u32, y: u32) -> Self {
        let position = Vec2::new(x as f32, y as f32);

        Self { position, entity }
    }
}

pub struct PlayerShipwreckEvent {
    pub ship_index: usize,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerTravelEvent>()
            .add_event::<PlayerShipwreckEvent>()
            .add_system(shipwreck_check::shipwreck_check)
            .add_system(shipwreck_remove::shipwreck_remove)
            .add_system(handle_travel::handle_travel);
    }
}
