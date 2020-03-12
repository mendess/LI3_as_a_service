use std::convert::TryFrom;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Month {
    Jan = 1,
    Feb = 2,
    Mar = 3,
    Apr = 4,
    May = 5,
    Jun = 6,
    Jul = 7,
    Aug = 8,
    Sep = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12,
}

impl Month {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Month {
    type Error = ();

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        use self::Month::*;
        match n {
            1 => Ok(Jan),
            2 => Ok(Feb),
            3 => Ok(Mar),
            4 => Ok(Apr),
            5 => Ok(May),
            6 => Ok(Jun),
            7 => Ok(Jul),
            8 => Ok(Aug),
            9 => Ok(Sep),
            10 => Ok(Oct),
            11 => Ok(Nov),
            12 => Ok(Dec),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}
