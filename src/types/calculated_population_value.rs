use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq, Clone)]
// TODO: validate entries here when loading data
pub struct CalculatedPopulationValue(HashMap<String, f32>);

impl CalculatedPopulationValue {
    pub fn value(&self, populations: &Vec<String>) -> f32 {
        self.0.iter().fold(0.0, |acc, (pop, modifier)| {
            let count = if pop == "Population" {
                populations.len()
            } else {
                populations.iter().filter(|i| i == &pop).count()
            } as f32;

            acc + modifier * count
        })
    }
}
