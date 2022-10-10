use crate::{
    game_state::{GameState, SettlementState},
    game_time::GameTimeAdvancedEvent,
    player::TransportType,
};
use bevy::prelude::*;
use serde::{Deserialize, Deserializer};

mod shipyard_ui;
mod temple_ui;

#[derive(Deserialize, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    pub entity: Option<Entity>,
}

impl From<BuildingType> for Building {
    fn from(building_type: BuildingType) -> Self {
        Self {
            building_type,
            entity: None,
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Temple {
    pub info: TempleInfo,
}

impl From<TempleInfo> for Temple {
    fn from(info: TempleInfo) -> Self {
        Self { info, ..default() }
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TempleInfo {
    pub deity: String,
}

#[derive(Component, Debug, Default)]
pub struct Shipyard {
    construction: Option<TransportType>,
    construction_time: u32,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum BuildingType {
    Shipyard,
    Temple(TempleInfo),
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
        app.add_system(shipyard_construction)
            .add_system_set(
                // TODO: close by escape
                SystemSet::on_update(GameState::Settlement(SettlementState::Shipyard))
                    .with_system(shipyard_ui::shipyard_ui),
            )
            .add_system_set(
                // TODO: close by escape
                SystemSet::on_update(GameState::Settlement(SettlementState::Temple))
                    .with_system(temple_ui::temple_ui),
            );
    }
}
