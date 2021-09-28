use anyhow::Result;
use std::collections::HashMap;

pub trait Getter {
    fn get(&self, query: &HashMap<&str, &str>) -> Result<String>;
}

pub fn new(url:&str) -> impl Getter {
    GetterImpl::new(url)
}

struct GetterImpl {
    url: String,
}

impl GetterImpl {
    fn new(url:&str) -> GetterImpl {
        GetterImpl{url:url.to_string()}
    }
}

impl Getter for GetterImpl {
    fn get(&self, query: &HashMap<&str, &str>) -> Result<String>{
        let mut req = ureq::get(&self.url);
        for (k, v) in query {
            req = req.query(k, v);
        }
        let resp = req.call()?;
        Ok(resp.into_string()?)
    }
}