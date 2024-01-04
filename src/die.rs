use std::convert::TryFrom;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub enum Die {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

pub enum DieConstructionError {
    NonDigit,
    OutOfRange,
}

impl TryFrom<String> for Die {
    type Error = DieConstructionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let num: u8 = value.parse().map_err(|_| DieConstructionError::NonDigit)?;

        match num {
            1 => Ok(Die::One),
            2 => Ok(Die::Two),
            3 => Ok(Die::Three),
            4 => Ok(Die::Four),
            5 => Ok(Die::Five),
            6 => Ok(Die::Six),
            _ => Err(DieConstructionError::OutOfRange),
        }
    }
}

impl Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl Debug for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Die({})", *self as u8)
    }
}
