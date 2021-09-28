use anyhow::Result;
use serde::{Deserialize};
use super::api::Response;

#[derive(Deserialize)]
struct Schema {
    #[serde(rename(deserialize = "Symbol"))]
    symbol: String,
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "EPS"))]
    eps: String,
}

pub fn parse(json:&str) -> Result<Response>{
    let sch: Schema = serde_json::from_str(json)?;
    let eps: f64 = sch.eps.parse::<f64>()?;
    let resp = Response::new(&sch.symbol, &sch.name, eps);
    Ok(resp)
}
