use std::convert::TryFrom;
use std::fmt::{Debug, Display};

const DICE_COUNT: u8 = 6;

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

impl Display for DieConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfRange => write!(f, "Face value out of range (1 - 6)."),
            Self::NonDigit => write!(f, "Die value not a digit."),
        }
    }
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

#[derive(Debug)]
pub struct Roll {
    dice: [Die; 6],
}

pub enum RollConstructionError {
    WrongDieCount,
    InvalidDie(DieConstructionError),
}

impl TryFrom<&[String]> for Roll {
    type Error = RollConstructionError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        if value.len() != DICE_COUNT.into() {
            return Err(RollConstructionError::WrongDieCount);
        }

        let mut current_roll: Roll = Roll { dice: [Die::One; 6] };
        for (i, n) in value.iter().enumerate() {
            current_roll.dice[i] = match n.clone().try_into() {
                Ok(d) => d,
                Err(e) => return Err(RollConstructionError::InvalidDie(e)),
            };
        }
        Ok(current_roll)
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;
        for die in self.dice {
            write!(f, "{die} ")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
