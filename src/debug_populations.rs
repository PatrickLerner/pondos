use std::{fs::File, io::BufReader};

use crate::{population::Population, settlement::Resource};

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

            let base_price = if production.resource == "Gold" {
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

        lines.push(format!("{} {}", population.name, output));
    }

    lines.sort();

    for line in lines.into_iter() {
        println!(" - {}", line);
    }
}
