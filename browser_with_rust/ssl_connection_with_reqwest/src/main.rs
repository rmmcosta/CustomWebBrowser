use reqwest::Url;
use reqwest::blocking::Client;

fn main() {
    let url = Url::parse("https://example.com").unwrap();
    let client = Client::builder().build().unwrap();
    let response = client.get(url).send().unwrap();
    println!("{}", response.text().unwrap());
}