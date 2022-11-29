fn main() {
    let response = reqwest::blocking::get("https://crypto.com/price")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let name = scraper::Selector::parse(".css-rkws3").unwrap();
    let price = scraper::Selector::parse(".css-b1ilzc").unwrap();
    let change = scraper::Selector::parse(".css-yyku61").unwrap();

    let title_selector = scraper::Selector::parse(".css-1nh9lk8").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

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

    titles.zip(1..101).for_each(|(item, number)| {
        if number % 2 == 0 {
            println!("Market cap: {}", item);
        } else {
            println!("24H Volume: {}", item);
        }
    });
}
