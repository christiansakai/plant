extern crate scraper;

use self::scraper::{Html, Selector, ElementRef};

#[derive(Debug)]
pub struct HomeInfo {
    pub link: String,
    pub name: String,
    pub difficulty: String,
    pub lighting: String,
    pub tank_placement: String,
    pub plant_structure: String,
    pub family: String,
    pub genus: String,
    pub size: String,
    pub growth_rate: String,
    pub region: String,
    pub location: String,
}

pub fn parse(html: &str) -> Vec<HomeInfo> {
    let document = Html::parse_document(html);

    let table_selector = Selector::parse("table#tablepress-2").unwrap();
    let tbody_selector = Selector::parse("tbody.row-hover").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();

    let table = document.select(&table_selector).next().unwrap();
    let tbody = table.select(&tbody_selector).next().unwrap();
    let trs = tbody.select(&tr_selector);

    trs.map(parse_row).collect()
}

fn parse_row(tr: ElementRef) -> HomeInfo {
    let link_with_nameselector = Selector::parse("td.column-1 > a").unwrap();
    let difficulty_selector = Selector::parse("td.column-2").unwrap();
    let lighting_selector = Selector::parse("td.column-3").unwrap();
    let tank_placement_selector = Selector::parse("td.column-4").unwrap();
    let plant_structure_selector = Selector::parse("td.column-5").unwrap();
    let family_selector = Selector::parse("td.column-6").unwrap();
    let genus_selector = Selector::parse("td.column-7").unwrap();
    let size_selector = Selector::parse("td.column-8").unwrap();
    let growth_rate_selector = Selector::parse("td.column-9").unwrap();
    let region_selector = Selector::parse("td.column-10").unwrap();
    let location_selector = Selector::parse("td.column-11").unwrap();

    let link_with_name = tr.select(&link_with_nameselector).next().unwrap();

    let link = link_with_name.value().attr("href").unwrap().to_string();
    let name = link_with_name.inner_html();
    let difficulty = tr.select(&difficulty_selector).next().unwrap().inner_html();
    let lighting = tr.select(&lighting_selector).next().unwrap().inner_html();
    let tank_placement = tr.select(&tank_placement_selector).next().unwrap().inner_html();
    let plant_structure = tr.select(&plant_structure_selector).next().unwrap().inner_html();
    let family = tr.select(&family_selector).next().unwrap().inner_html();
    let genus = tr.select(&genus_selector).next().unwrap().inner_html();
    let size = tr.select(&size_selector).next().unwrap().inner_html();
    let growth_rate = tr.select(&growth_rate_selector).next().unwrap().inner_html();
    let region = tr.select(&region_selector).next().unwrap().inner_html();
    let location = tr.select(&location_selector).next().unwrap().inner_html();

    HomeInfo {
        link,
        name,
        difficulty,
        lighting,
        tank_placement,
        plant_structure,
        family,
        genus,
        size,
        growth_rate,
        region,
        location,
    }
}
