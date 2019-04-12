use crate::util::Month;
use std::convert::TryFrom;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Filial {
    One, Two, Three,
}

impl Filial {
    pub fn as_u8(self) -> u8 {
        use self::Filial::*;
        match self {
            One => 1,
            Two => 2,
            Three => 3,
        }
    }
}

impl TryFrom<u8> for Filial {
    type Error = ();
    fn try_from(s :u8) -> Result<Self, Self::Error> {
        use self::Filial::*;
        match s {
            1 => Ok(One),
            2 => Ok(Two),
            3 => Ok(Three),
            _ => Err(())
        }
    }
}

impl TryFrom<&str> for Filial {
    type Error = ();
    fn try_from(s :&str) -> Result<Self, Self::Error> {
        use self::Filial::*;
        match s {
            "1" => Ok(One),
            "2" => Ok(Two),
            "3" => Ok(Three),
            _ => Err(()),
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
            let m = Month::try_from(month).ok()?;
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

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn filial(&self) -> Filial {
        self.filial
    }

    pub fn total_price(&self) -> f64 {
        self.price * self.amount as f64
    }
}

impl std::fmt::Display for Sale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} {} {} {} {}",
               self.product, self.price, self.amount, if self.promotion { "P" } else { "N" },
               self.client, self.month, self.filial)
    }
}

impl std::fmt::Display for Filial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Filial::*;
        let to_str = || match self {
            One => "1",
            Two => "2",
            Three => "3",
        };
        write!(f, "{}", to_str())
    }
}
