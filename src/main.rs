mod company_overview;
mod http;

use anyhow::Result;
use clap::{load_yaml, App};
use crate::company_overview::api::Querier;

const URL: &str = "https://www.alphavantage.co/query";

fn main() {
    let api_key = get_api_key().expect("API_KEY environment variable must be defined");
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();
    if let Some(matches) = matches.subcommand_matches("company-overview"){
        let symbol = matches.value_of("symbol").unwrap();
        run_company_overview(&api_key,symbol);
    }
}

fn get_api_key() -> Result<String>{
    Ok(std::env::var("API_KEY")?)
}

fn run_company_overview(api_key:&str, symbol:&str){
    let getter = http::new(URL);
    let querier = company_overview::api::new(Box::new(getter));
    let resp = querier.query(api_key, symbol).unwrap();
    dbg!(resp);
}