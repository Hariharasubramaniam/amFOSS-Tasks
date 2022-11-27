fn main() {
    let response = reqwest::blocking::get("https://crypto.com/price")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let name = scraper::Selector::parse(".css-rkws3").unwrap();
    let price = scraper::Selector::parse(".css-b1ilzc").unwrap();
    let change = scraper::Selector::parse(".css-yyku61").unwrap();
    let volume = scraper::Selector::parse(".css-1nh9lk8").unwrap();
    let market_cap = scraper::Selector::parse(".css-1nh9lk8").unwrap();

    for element in document.select(&name) {
        println!("Name: {}", element.text().collect::<Vec<_>>().join(""));
    }

    for element in document.select(&price) {
        println!("Price: {}", element.text().collect::<Vec<_>>().join(""));
    }

    for element in document.select(&change) {
        println!(
            "24h Change: {}",
            element.text().collect::<Vec<_>>().join("")
        );
    }

    for element in document.select(&volume) {
        println!(
            "24h Volume: {}",
            element.text().collect::<Vec<_>>().join("")
        );
    }

    for element in document.select(&market_cap) {
        println!(
            "Market Cap: {}",
            element.text().collect::<Vec<_>>().join("")
        );
    }

    //writing to a csv file\
}
