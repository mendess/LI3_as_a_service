use crate::util::Month;

#[derive(Debug,Clone,Copy)]
pub enum Filial {
    One, Two, Three, Error
}

impl From<&str> for Filial {
    fn from(s :&str) -> Self {
        use self::Filial::*;
        match s {
            "1" => One,
            "2" => Two,
            "3" => Three,
            _ => Error,
        }
    }
}

#[derive(Debug)]
pub struct Sale {
    product: String,
    client: String,
    price: f64,
    amount: u32,
    promotion: bool,
    month: Month,
    filial: Filial,
}

#[allow(dead_code)]
impl Sale {
    pub fn new(product: String,
               client: String,
               price: f64,
               amount: u32,
               promotion: bool,
               month: u8,
               filial: Filial
              ) -> Option<Self> {
        if price < 0.0 || price > 999.99 {
            None
        } else {
            let m = Month::from(month);
            Some(Sale { product, client, price, amount, promotion, month: m, filial })
        }
    }

    pub fn client(&self) -> &str {
        &self.client
    }

    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn month(&self) -> Month {
        self.month
    }

    pub fn promotion(&self) -> bool {
        self.promotion
    }

    pub fn filial(&self) -> Filial {
        self.filial
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn price(&self) -> f64 {
        self.price
    }
}
