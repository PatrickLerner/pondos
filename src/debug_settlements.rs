use crate::{population::Population, resources::Resource, types::Settlement, COIN_NAME};
use std::{fs::File, io::BufReader};

pub fn debug_settlements() {
    let resources: Vec<Resource> = {
        let file = File::open("assets/game.resources").unwrap();
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).unwrap()
    };
    let populations: Vec<Population> = {
        let file = File::open("assets/game.populations").unwrap();
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).unwrap()
    };
    let mut settlements: Vec<Settlement> = {
        let file = File::open("assets/game.settlements").unwrap();
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).unwrap()
    };

    settlements.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    println!("Debug yearly value of settlement production:\n");

    let mut lines = vec![];

    for settlement in settlements.iter() {
        let mut output = 0;

        for population in settlement.populations.iter() {
            let population = populations.iter().find(|p| p.name == *population).unwrap();
            for production in population.production.iter() {
                let yearly = 2 * production.amount.growth
                    + production.amount.summer
                    + production.amount.harvest
                    + 2 * production.amount.winter;

                let base_price = if production.resource == COIN_NAME {
                    1
                } else {
                    resources
                        .iter()
                        .find(|i| i.name == production.resource)
                        .unwrap()
                        .base_price
                };

                output += yearly * base_price;
            }
        }

        lines.push(format!(
            "{} {} silver per year (pop: {})",
            settlement.name,
            output,
            settlement.populations.len()
        ));
    }

    lines.sort();

    for line in lines.into_iter() {
        println!(" - {}", line);
    }
}
