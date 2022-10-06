use crate::game_time::GameTime;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Component, Debug, Hash, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SeasonalAmount<T> {
    #[serde(default)]
    pub growth: T,
    #[serde(default)]
    pub summer: T,
    #[serde(default)]
    pub harvest: T,
    #[serde(default)]
    pub winter: T,
}

impl<T> SeasonalAmount<T>
where
    T: Copy,
{
    pub fn value(&self, time: &GameTime) -> T {
        if time.is_growth_season() {
            self.growth
        } else if time.is_summer_season() {
            self.summer
        } else if time.is_harvest_season() {
            self.harvest
        } else if time.is_winter_season() {
            self.winter
        } else {
            unreachable!("unknown season");
        }
    }
}
