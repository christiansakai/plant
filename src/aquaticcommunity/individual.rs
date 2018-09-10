extern crate scraper;

use self::scraper::{Html, Selector, ElementRef};
use ::util;

#[derive(Debug)]
pub struct IndividualInfo {
    pub name: String,
    pub family: String,
    pub continent: String,
    pub country_of_origin: String,
    pub region: String,
    pub height: String,
    pub width: String,
    pub light_requirement: String,
    pub temperature: String,
    pub hardness_tolerance: String,
    pub ph_tolerance: String,
    pub growth: String,
    pub demands: String,
    pub description: String,
}

#[derive(Default)]
struct PropertyInfo {
    family: Option<String>,
    continent: Option<String>,
    region: Option<String>,
    country_of_origin: Option<String>,
    height: Option<String>,
    width: Option<String>,
    light_requirement: Option<String>,
    temperature: Option<String>,
    hardness_tolerance: Option<String>,
    ph_tolerance: Option<String>,
    growth: Option<String>,
    demands: Option<String>,
}


pub fn parse(html: &str) -> IndividualInfo {
    let document = Html::parse_document(html);

    let h2_selector = Selector::parse("h2").unwrap();
    let p_selector = Selector::parse("p").unwrap();
    let strong_selector = Selector::parse("strong").unwrap();

    let h2 = document.select(&h2_selector).next().unwrap();
    let mut ps = document.select(&p_selector);

    let name = h2.inner_html();

    ps.next().unwrap();

    let property_info = ps.next().unwrap().inner_html();

    let PropertyInfo {
        family,
        continent,
        region,
        country_of_origin,
        height,
        width,
        light_requirement,
        temperature,
        hardness_tolerance,
        ph_tolerance,
        growth,
        demands,
    } = clean_and_parse_property_info(property_info);

    let description = ps
        .next()
        .unwrap()
        .inner_html();

    let description = util::clean_string(description);

    IndividualInfo {
        name,
        family: family.unwrap_or_else(|| String::new()),
        continent: continent.unwrap_or_else(|| String::new()),
        region: region.unwrap_or_else(|| String::new()),
        country_of_origin: country_of_origin.unwrap_or_else(|| String::new()),
        height: height.unwrap_or_else(|| String::new()),
        width: width.unwrap_or_else(|| String::new()),
        light_requirement: light_requirement.unwrap_or_else(|| String::new()),
        temperature: temperature.unwrap_or_else(|| String::new()),
        hardness_tolerance: hardness_tolerance.unwrap_or_else(|| String::new()),
        ph_tolerance: ph_tolerance.unwrap_or_else(|| String::new()),
        growth: growth.unwrap_or_else(|| String::new()),
        demands: demands.unwrap_or_else(|| String::new()),
        description,
    }

}

fn clean_and_parse_property_info(property_info: String) -> PropertyInfo {
    let property_info = util::clean_string(property_info)
        .replace("<br>", "")
        .replace("\n", "");

    let temp: Vec<&str> = property_info
        .split("\n")
        // .split("<br>")
        // .split(":")
        .collect();
    
    println!("{:?}", temp);

    let mut property_info: PropertyInfo = Default::default(); 

    // for property in properties {
        // println!("{:?}", property);
        // let (key, value): (String, String) = get_key_value(property);

        // match key.as_ref() {
        //     "Family" => property_info.family = Some(value),
        //     "Continent" => property_info.continent = Some(value),
        //     "Region" => property_info.region = Some(value),
        //     "Country of origin" => property_info.country_of_origin = Some(value),
        //     "Height" => property_info.height = Some(value),
        //     "Width" => property_info.width = Some(value),
        //     "Light requirements" => property_info.light_requirement = Some(value),
        //     "Temperature" => property_info.temperature = Some(value),
        //     "Hardness tolerance" => property_info.hardness_tolerance = Some(value),
        //     "pH tolerance" => property_info.ph_tolerance = Some(value),
        //     "Growth" => property_info.growth = Some(value),
        //     "Demands" => property_info.demands = Some(value),
        //     &_ => (),
        // }
    // }

    property_info
}

fn get_key_value(property: &str) -> (String, String) {
    let property = util::clean_string(property.to_string());

    let key_value: Vec<&str> = property
        .split(":")
        .collect();

    (key_value[0].to_string(), util::clean_string(key_value[1].to_string()))
}

