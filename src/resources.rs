use crate::types::CalculatedPopulationValue;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Resource {
    pub name: String,
    pub base_price: u32,
    pub demand: CalculatedPopulationValue,
    pub max: CalculatedPopulationValue,
}
