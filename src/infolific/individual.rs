extern crate scraper;

use self::scraper::{Html, Selector, ElementRef};
use ::util;

#[derive(Debug)]
pub struct IndividualInfo {
    pub name: String,
    pub synonims: String,
    pub hardiness: String,
    pub light_needs: String,
    pub plant_structure: String,
    pub family: String,
    pub genus: String,
    pub region: String,
    pub location: String,
    pub size: String,
    pub growth_rate: String,
    pub can_be_grown_emersed: String,
    pub description: String,
}

#[derive(Default)]
struct PropertyInfo {
    synonims: Option<String>,
    hardiness: Option<String>,
    light_needs: Option<String>,
    plant_structure: Option<String>,
    family: Option<String>,
    genus: Option<String>,
    region: Option<String>,
    location: Option<String>,
    size: Option<String>,
    growth_rate: Option<String>,
    can_be_grown_emersed: Option<String>,
}

pub fn parse(html: &str) -> IndividualInfo {
    let document = Html::parse_document(html);

    let info_selector = Selector::parse("tr:nth-child(4) tbody").unwrap();
    let name_selector = Selector::parse("td:nth-child(1) td:nth-child(2) b font").unwrap();
    let property_info_selector = Selector::parse("tr:nth-child(3) td font").unwrap();
    let description_selector = Selector::parse("tr:nth-child(5) td font").unwrap();

    let info = document.select(&info_selector).next().unwrap();

    let name = info
        .select(&name_selector)
        .next()
        .unwrap()
        .inner_html();

    let name = clean_name(name);

    let property_info = info.select(&property_info_selector).next().unwrap().inner_html();

    let PropertyInfo {
        synonims,
        hardiness,
        light_needs,
        plant_structure,
        family,
        genus,
        region,
        location,
        size,
        growth_rate,
        can_be_grown_emersed,
    } = clean_and_parse_property_info(property_info);

    let description = info
        .select(&description_selector)
        .next()
        .unwrap()
        .inner_html();

    let description = clean_description(description);

    IndividualInfo {
        name,
        synonims: synonims.unwrap_or_else(|| String::new()),
        hardiness: hardiness.unwrap_or_else(|| String::new()),
        light_needs: light_needs.unwrap_or_else(|| String::new()),
        plant_structure: plant_structure.unwrap_or_else(|| String::new()),
        family: family.unwrap_or_else(|| String::new()),
        genus: genus.unwrap_or_else(|| String::new()),
        region: region.unwrap_or_else(|| String::new()),
        location: location.unwrap_or_else(|| String::new()),
        size: size.unwrap_or_else(|| String::new()),
        growth_rate: growth_rate.unwrap_or_else(|| String::new()),
        can_be_grown_emersed: can_be_grown_emersed.unwrap_or_else(|| String::new()),
        description,
    }
}

fn clean_name(name: String) -> String {
    name.trim().to_string()
}

fn clean_description(description: String) -> String {
    let trimmed = description
        .trim()
        .to_string()
        .replace("<br>", "");

    let cleaned_strs: Vec<&str> = trimmed
        .split("\n")
        .collect();

    let mut result = String::new();

    for clean_str in cleaned_strs.iter().take(cleaned_strs.len() - 1) {
        result.push_str(clean_str);
        result.push_str(" ");
    }

    result.trim().to_string()
}

fn clean_and_parse_property_info(property_info: String) -> PropertyInfo {
    let properties: Vec<&str> = property_info
        .trim()
        .split("<br>")
        .collect();

    let mut property_info: PropertyInfo = Default::default(); 

    for property in properties {
        let (key, value): (String, String) = get_key_value(property);

        match key.as_ref() {
            "synonims" => property_info.synonims = Some(value),
            "hardiness" => property_info.hardiness = Some(value),
            "light needs" => property_info.light_needs = Some(value),
            "plant structure" => property_info.plant_structure = Some(value),
            "family" => property_info.family = Some(value),
            "genus" => property_info.genus = Some(value),
            "region" => property_info.region = Some(value),
            "location" => property_info.location = Some(value),
            "size" => property_info.size = Some(value),
            "growth rate" => property_info.growth_rate = Some(value),
            "can be grown emersed" => property_info.can_be_grown_emersed = Some(value),
            &_ => (),
        }
    }

    property_info
}

fn get_key_value(property: &str) -> (String, String) {
    let property = util::clean_string(property.to_string());

    let key_value: Vec<&str> = property
        .split(":")
        .collect();

    (key_value[0].to_string(), key_value[1].to_string())
}

