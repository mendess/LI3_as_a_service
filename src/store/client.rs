use super::sale::Filial;

#[derive(Debug,Clone)]
pub struct Client{
    id: String,
    purchases: (u32, u32, u32),
}

use regex::Regex;
use lazy_static::lazy_static;


impl Client {
    pub fn new(code: String) -> Option<Self>  {
        lazy_static! {
            static ref RE :Regex = Regex::new(r"[A-Z][1-9]\d{3}").unwrap();
        }
        if RE.is_match_at(&code, 0) && code.len() == 5 {
            Some(Client{ id: code, purchases: (0,0,0) })
        } else {
            None
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn make_purchase(&mut self, filial: Filial) {
        use super::sale::Filial::*;
        match filial {
            One => self.purchases.0 += 1,
            Two => self.purchases.1 += 1,
            Three => self.purchases.2 += 1,
        }
    }

    pub fn purchases(&self) -> (u32, u32, u32) {
        self.purchases
    }
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
