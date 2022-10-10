use crate::{
    game_state::{GameState, SettlementState},
    game_time::GameTimeAdvancedEvent,
    player::TransportType,
};
use bevy::prelude::*;
use serde::{Deserialize, Deserializer};

mod shipyard_ui;

#[derive(Deserialize, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    pub entity: Entity,
}

impl From<BuildingType> for Building {
    fn from(building_type: BuildingType) -> Self {
        Self {
            building_type,
            entity: Entity::from_raw(0),
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Shipyard {
    construction: Option<TransportType>,
    construction_time: u32,
}

#[derive(Deserialize, Debug)]
pub enum BuildingType {
    Shipyard,
}

pub fn building_deserialize<'de, D>(deserializer: D) -> Result<Vec<Building>, D::Error>
where
    D: Deserializer<'de>,
{
    let building_types = Vec::<BuildingType>::deserialize(deserializer)?;

    Ok(building_types
        .into_iter()
        .map(|building_type| building_type.into())
        .collect())
}

fn shipyard_construction(
    mut shipyards: Query<&mut Shipyard>,
    mut events: EventReader<GameTimeAdvancedEvent>,
) {
    for _ in events.iter() {
        for mut shipyard in shipyards.iter_mut() {
            if shipyard.construction_time > 0 {
                shipyard.construction_time -= 1;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SelectedBuilding(pub Entity);

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shipyard_construction).add_system_set(
            // TODO: close by escape
            SystemSet::on_update(GameState::Settlement(SettlementState::Shipyard))
                .with_system(shipyard_ui::shipyard_ui),
        );
    }
}
