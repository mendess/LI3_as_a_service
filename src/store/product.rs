#[derive(Debug,Clone)]
pub struct Product(String);

use regex::Regex;
use lazy_static::lazy_static;

impl Product {
    pub fn new(code: String) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[A-Z][A-Z][1-9]\d{3}").unwrap();
        }
        if RE.is_match_at(&code, 0) && code.len() == 6 {
            Some(Product(code))
        } else {
            None
        }
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
