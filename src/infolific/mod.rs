extern crate reqwest;
extern crate threadpool;

mod parse_home;
mod parse_individual;

use std::sync::mpsc::channel;

use self::threadpool::ThreadPool;

use util;

const URL: &'static str = "https://infolific.com/pets/freshwater-aquariums/plant-database/";
const WORKERS: usize = 4;

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

    let home_infos = parse_home::parse(&body);

    for home_info in home_infos {
        let tx = tx.clone();
        let link = home_info.link.clone();

        pool.execute(move || {
            let body = reqwest::get(&link)
                .unwrap()
                .text()
                .unwrap();

            let individual_info = parse_individual::parse(&body);

            tx.send(individual_info).unwrap();
        });
    }

    for result in rx {
        println!("{:#?}", result);
    }

    pool.join();
}
