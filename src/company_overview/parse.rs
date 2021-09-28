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

#[cfg(test)]
mod tests {
    use crate::company_overview::api::Response;
    use std::io::Read;
    #[test]
    fn test_parse(){
        let mut f = std::fs::File::open("./src/company_overview/example.json").unwrap();
        let mut json = String::new();
        f.read_to_string(&mut json).unwrap();
        let want = Response::new("IBM", "International Business Machines Corporation", 5.92);
        let got = super::parse(&json).unwrap();
        assert_eq!(want, got);
    }
}