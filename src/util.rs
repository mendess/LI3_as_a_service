#[derive(PartialEq,Eq,PartialOrd,Ord,Debug,Clone,Copy)]
pub enum Month {
    Jan = 1,
    Fev = 2,
    Mar = 3,
    Apr = 4,
    May = 5,
    Jun = 6,
    Jul = 7,
    Aug = 8,
    Sep = 9,
    Out = 10,
    Nov = 11,
    Dez = 12,
}

impl Month {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl From<u8> for Month {
    fn from(n :u8) -> Self {
        use self::Month::*;
        match n {
            1 => Jan,
            2 => Fev,
            3 => Mar,
            4 => Apr,
            5 => May,
            6 => Jun,
            7 => Jul,
            8 => Aug,
            9 => Sep,
            10 => Out,
            11 => Nov,
            12 => Dez,
            _ => panic!("Invalid month number: {}", n),
        }
    }
}
