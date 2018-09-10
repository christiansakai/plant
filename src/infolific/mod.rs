extern crate reqwest;
extern crate threadpool;

mod home;
mod individual;

use std::sync::mpsc::channel;
use self::threadpool::ThreadPool;


const URL: &'static str = "https://infolific.com/pets/freshwater-aquariums/plant-database/";
const WORKERS: usize = 4;

#[derive(Debug)]
pub struct Plant {
    name: String,
    synonims: String,
    difficulty: String,
    lighting: String,
    tank_placement: String,
    plant_structure: String,
    family: String,
    genus: String,
    size: String,
    growth_rate: String,
    region: String,
    location: String,
    hardiness: String,
    can_be_grown_emersed: String,
    description: String,
}

pub fn scrape() {
    let pool = ThreadPool::new(WORKERS);
    let (tx, rx) = channel();

    let body = reqwest::get(URL)
        .unwrap()
        .text()
        .unwrap();

    let home_infos = home::parse(&body);

    for home_info in home_infos {
        let tx = tx.clone();

        pool.execute(move || {
            let body = reqwest::get(&home_info.link)
                .unwrap()
                .text()
                .unwrap();

            let individual_info = individual::parse(&body);

            let plant_info = Plant {
                name: home_info.name,
                synonims: individual_info.synonims,
                difficulty: home_info.difficulty,
                lighting: home_info.lighting,
                tank_placement: home_info.tank_placement,
                plant_structure: home_info.plant_structure,
                family: home_info.family,
                genus: home_info.genus,
                size: home_info.size,
                growth_rate: home_info.growth_rate,
                region: home_info.region,
                location: home_info.location,
                hardiness: individual_info.hardiness,
                can_be_grown_emersed: individual_info.can_be_grown_emersed,
                description: individual_info.description,
            };

            tx.send(plant_info).unwrap();
        });
    }

    drop(tx);

    for result in rx {
        println!("{:#?}", result);
    }
}
