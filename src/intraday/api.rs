use crate::http::Getter;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub meta_data: HashMap<String, String>,
    pub time_series: HashMap<String, StockValue>,
}

impl Response {
    pub fn new(
        meta_data: HashMap<String, String>,
        time_series: HashMap<String, StockValue>,
    ) -> Response {
        Response {
            meta_data,
            time_series,
        }
    }
}

#[derive(Debug)]
pub struct StockValue {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

impl StockValue {
    pub fn new(open: f64, high: f64, low: f64, close: f64, volume: i64) -> StockValue {
        StockValue {
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

pub trait Querier {
    fn query(&self, api_key: &str, symbol: &str) -> Result<Response>;
}

pub fn new(getter: Box<dyn Getter>) -> impl Querier {
    QuerierImpl::new(getter)
}

struct QuerierImpl {
    getter: Box<dyn Getter>,
}

impl QuerierImpl {
    fn new(getter: Box<dyn Getter>) -> QuerierImpl {
        QuerierImpl { getter }
    }
}

impl Querier for QuerierImpl {
    fn query(&self, api_key: &str, symbol: &str) -> Result<Response> {
        let mut query: HashMap<&str, &str> = HashMap::new();
        query.insert("function", "TIME_SERIES_INTRADAY");
        query.insert("symbol", symbol);
        query.insert("interval", "5min");
        query.insert("apikey", api_key);
        let resp_str = self.getter.get(&query)?;
        let resp = super::parse::parse(&resp_str)?;
        Ok(resp)
    }
}
