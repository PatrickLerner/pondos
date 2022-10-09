use crate::{population::Population, resources::Resource, settlement::Settlement, COIN_NAME};
use std::{fs::File, io::BufReader};

pub fn debug_populations() {
    let resources: Vec<Resource> = {
        let file = File::open("assets/game.resources").unwrap();
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).unwrap()
    };
    let mut populations: Vec<Population> = {
        let file = File::open("assets/game.populations").unwrap();
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).unwrap()
    };
    let settlements: Vec<Settlement> = {
        let file = File::open("assets/game.settlements").unwrap();
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).unwrap()
    };

    let all_pops: Vec<String> = settlements
        .into_iter()
        .flat_map(|settlement| settlement.populations)
        .collect();

    populations.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

    println!("Debug yearly value of population production:\n");

    let mut lines = vec![];

    for population in populations.iter() {
        let mut output = 0;

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

        let count = all_pops.iter().filter(|p| **p == population.name).count();

        lines.push(format!(
            "{} {} silver per year ({}x)",
            population.name, output, count
        ));
    }

    lines.sort();

    for line in lines.into_iter() {
        println!(" - {}", line);
    }
}
