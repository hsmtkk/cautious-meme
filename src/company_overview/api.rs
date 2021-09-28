use anyhow::Result;
use crate::http::Getter;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Response {
    pub symbol: String,
    pub name: String,
    pub eps: f64,
}

impl Response {
    pub fn new(symbol:&str, name:&str, eps:f64) -> Response {
        Response{symbol:symbol.to_string(), name:name.to_string(), eps}
    }
}

pub trait Querier {
    fn query(&self, api_key: &str, symbol:&str) -> Result<Response>;
}

pub fn new(getter:Box<dyn Getter>) -> impl Querier {
    QuerierImpl::new(getter)
}

struct QuerierImpl {
    getter: Box<dyn Getter>,
}

impl QuerierImpl {
    fn new(getter:Box<dyn Getter>) -> QuerierImpl {
        QuerierImpl{getter}
    }
}

impl Querier for QuerierImpl {
    fn query(&self, api_key: &str, symbol:&str) -> Result<Response>{
        let mut query: HashMap<&str, &str> = HashMap::new();
        query.insert("function", "OVERVIEW");
        query.insert("symbol", symbol);
        query.insert("apikey", api_key);
        let resp_str = self.getter.get(&query)?;
        let resp = super::parse::parse(&resp_str)?;
        Ok(resp)
    }
}
