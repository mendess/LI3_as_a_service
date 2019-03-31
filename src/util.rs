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
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn from(n :u8) -> Option<Self> {
        use self::Month::*;
        match n {
            1 => Some(Jan),
            2 => Some(Fev),
            3 => Some(Mar),
            4 => Some(Apr),
            5 => Some(May),
            6 => Some(Jun),
            7 => Some(Jul),
            8 => Some(Aug),
            9 => Some(Sep),
            10 => Some(Out),
            11 => Some(Nov),
            12 => Some(Dez),
            _ => None,
        }
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}
