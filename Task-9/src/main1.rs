fn main() {
    let response = reqwest::blocking::get("https://crypto.com/price")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse(".css-1nh9lk8").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles.zip(1..101).for_each(|(item, number)| {
        if number % 2 == 0 {
            println!("Market cap: {}", item);
        } else {
            println!("24H Volume: {}", item);
        }
    });
}
