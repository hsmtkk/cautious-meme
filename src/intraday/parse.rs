use anyhow::Result;
use serde::{Deserialize};
use std::collections::HashMap;
use super::api::{Response, StockValue};

#[derive(Deserialize)]
struct StockValueSchema {
    #[serde(rename(deserialize = "1. open"))]    
    open: String,
    #[serde(rename(deserialize = "2. high"))]    
    high: String,
    #[serde(rename(deserialize = "3. low"))]    
    low: String,
    #[serde(rename(deserialize = "4. close"))]    
    close: String,
    #[serde(rename(deserialize = "5. volume"))]    
    volume: String,
}

#[derive(Deserialize)]
struct Schema {
    #[serde(rename(deserialize = "Meta Data"))]
    meta_data: HashMap<String, String>,
    #[serde(rename(deserialize = "Time Series (5min)"))]
    time_series: HashMap<String, StockValueSchema>,
}

pub fn parse(json:&str) -> Result<Response>{
    let sch: Schema = serde_json::from_str(json)?;
    let mut time_series: HashMap<String, StockValue> = HashMap::new();
    for (k, v) in &sch.time_series {
        let op: f64 = v.open.parse::<f64>()?;
        let hi: f64 = v.high.parse::<f64>()?;
        let lo: f64 = v.low.parse::<f64>()?;
        let cl: f64 = v.close.parse::<f64>()?;
        let vo: i64 = v.volume.parse::<i64>()?;
        time_series.insert(k.to_string(), StockValue::new(op, hi, lo, cl, vo));
    }
    let resp = Response::new(sch.meta_data, time_series);
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use crate::intraday::api::Response;
    use std::io::Read;
    #[test]
    fn test_parse(){
        let mut f = std::fs::File::open("./src/intraday/example.json").unwrap();
        let mut json = String::new();
        f.read_to_string(&mut json).unwrap();
        let got = super::parse(&json).unwrap();
        assert_eq!("Intraday (5min) open, high, low, close prices and volume", got.meta_data.get("1. Information").unwrap());
    }
}