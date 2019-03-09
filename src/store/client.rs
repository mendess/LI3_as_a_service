#[derive(Debug,Clone)]
pub struct Client(String);

use regex::Regex;
use lazy_static::lazy_static;


impl Client {
    pub fn new(code: String) -> Option<Self>  {
        lazy_static! {
            static ref re :Regex = Regex::new(r"[A-Z][1-9]\d{3}").unwrap();
        }
        if re.is_match(&code) {
            Some(Client(code))
        } else {
            None
        }
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Client {
    fn from(s :&str) -> Self {
        Client(s.into())
    }
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
