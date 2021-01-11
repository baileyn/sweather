use dotenv::dotenv;
use weatherstack::*;

fn main() {
    dotenv().ok();

    let api = WeatherStack::new(
        std::env::var("API_ACCESS_KEY").expect("Expected API_ACCESS_KEY to be defined."),
    );

    let data = api.current("Oceana, WV".into());

    println!("{:#?}", data);
}
