use std::collections::HashMap;
use std::io;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct CurrencyWrapper {
    currencies: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug)]
struct RateWrapper {
    rates:  HashMap<String, HashMap<String, f32>>
}

fn main() {

    let currency_wrapper = initialize_currencies();
    let rate_wrapper = initialize_rates();
    println!("Initialization complete.");


    println!("Enter source currency");
    let source_currency = get_currency();
    validate_currency_is_supported(&source_currency, &currency_wrapper);

    println!("Enter destination currency");
    let destination_currency = get_currency();
    validate_currency_is_supported(&destination_currency, &currency_wrapper);


    let destination_currency_map_option = rate_wrapper.rates.get(&source_currency);

    if let None = destination_currency_map_option {
        panic!("No exchage rate data for {}", &source_currency);
    }

    if let Some(destination_currency_map) = destination_currency_map_option {
        if let Some(exchange_rate) = destination_currency_map.get(&destination_currency) {
            println!("Exchange rate is {}", &exchange_rate);
        } else {
            panic!("No exchange rate data for {} to {}", &source_currency, &destination_currency);
        }

    }

    //TODO retrieve amount to exchange




}

fn get_currency() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    input.trim().to_string()
}

fn initialize_currencies() -> CurrencyWrapper {
    println!("Initializing currencies...");
    let json_string = fs::read_to_string("./currencies.json").unwrap_or_else(|error| {
        eprintln!("An error occured while initializing currencies");
        panic!("{:?}", error);
    });

   serde_json::from_str(&json_string).unwrap()
}

fn initialize_rates() -> RateWrapper {
    println!("Initializing rates...");
    let json_string = fs::read_to_string("./rates.json").unwrap_or_else(|error| {
        println!("An error occured while initializing rates");
        panic!("{:?}", error);
    });

    serde_json::from_str(&json_string).unwrap()
}

fn validate_currency_is_supported(currency: &String, currencies: &CurrencyWrapper) {
    if !currencies.currencies.contains_key(currency) {
        panic!("{} is not a supported currency", &currency);
    }
}
