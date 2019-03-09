#[derive(Debug,Clone)]
pub struct Product(String);

use regex::Regex;
use lazy_static::lazy_static;

impl Product {
    pub fn new(code: String) -> Option<Self> {
        lazy_static! {
            static ref re: Regex = Regex::new("...").unwrap();
        }
        if re.is_match(&code) {
            Some(Product(code))
        } else {
            None
        }
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Product {
    fn from(s :&str) -> Self {
        Product(s.into())
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
