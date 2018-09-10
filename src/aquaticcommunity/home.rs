extern crate scraper;

use self::scraper::{Html, Selector, ElementRef};

#[derive(Debug)]
pub struct HomeInfo {
    pub link: String,
    pub name: String,
}

pub fn parse(html: &str) -> Vec<HomeInfo> {
    let document = Html::parse_document(html);

    let center_selector = Selector::parse("center").unwrap();
    let p_selector = Selector::parse("p").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let center = document.select(&center_selector).next().unwrap();
    let ps = center.select(&p_selector);

    let mut home_infos = Vec::new();

    for p in ps {
        let mut a = p.select(&a_selector);
        
        while let Some(a_href) = a.next() {
            let name = a_href.inner_html();
            let link = a_href
                .value()
                .attr("href")
                .unwrap()
                .to_string();

            home_infos.push(HomeInfo {
                name,
                link,
            });
        }
    }

    home_infos
}
