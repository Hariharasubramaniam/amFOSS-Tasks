use csv;
use std::fs::OpenOptions;

fn main() {
    let mut wtr = csv::Writer::from_writer(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open("Crypto.csv")
            .unwrap(),
    );
    let response = reqwest::blocking::get("https://crypto.com/price")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let mut data = Vec::new();

    let name = scraper::Selector::parse(".css-rkws3").unwrap();
    let names = document.select(&name).map(|x| x.inner_html());
    names.zip(1..51).for_each(|(_name, element)| {
        let rows = vec![_name];
        data.push(rows);
    });

    let price = scraper::Selector::parse(".css-b1ilzc").unwrap();
    let prices = document.select(&price).map(|x| x.inner_html());
    prices.zip(1..51).for_each(|(_price, element)| {
        data[element - 1].push(_price);
    });
    let change = scraper::Selector::parse("td.css-1b7j986>p").unwrap();
    let changes = document.select(&change).map(|x| x.inner_html());
    changes.zip(1..51).for_each(|(_change, element)| {
        data[element - 1].push(_change);
    });

    let mut loop_count = 0;
    let title_selector = scraper::Selector::parse(".css-1nh9lk8").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles.zip(1..101).for_each(|(item, number)| {
        if number % 2 == 0 {
            data[loop_count].push(item);
            loop_count += 1;
        } else {
            data[loop_count].push(item);
        }
    });
    wtr.write_record(&[
        "NAME ",
        "PRICE ",
        "24H CHANGE ",
        "24H VOLUME ",
        "MARKET CAP ",
    ]);
    for row in data.iter() {
        wtr.write_record(row).unwrap();
    }
}
