use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::iter::zip;

pub const DICE_COUNT: u8 = 6;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Die {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Any = 7,
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
        num.try_into()
    }
}

impl TryFrom<u8> for Die {
    type Error = DieConstructionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
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
    dice: [Die; DICE_COUNT as usize],
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

impl Roll {
    /// Constructs a new roll containing the given set of dice.
    pub fn new(dice: [Die; 6]) -> Self {
        Self { dice }
    }

    /// Counts the occurrences of each die face
    fn die_counts(&self) -> HashMap<Die, u8> {
        let mut counts = HashMap::from_iter(zip(self.dice.iter().cloned(), [0u8; 6].iter().cloned()));
        for die in self.dice {
            counts.entry(die).and_modify(|c| *c += 1);
        }
        counts
    }

    /// Calculates the score of the roll
    pub fn score(&self) -> u32 {
        let counts = self.die_counts();

        // Consider straight (dice 1 - 6)
        if counts.keys().len() == 6 {
            return 2000;
        }

        // Consider three pairs
        if counts.keys().len() == 3 && counts.values().all(|c| *c == 2) {
            return 1500;
        }

        let mut score: u32 = 0;
        for (die, count) in counts.iter() {
            match (die, *count) {
                (Die::One, 1 | 2) => score += *count as u32 * 100,
                (Die::One, _) => score += 1000 * 2u32.pow(*count as u32 - 3),
                (Die::Five, 1 | 2) => score += *count as u32 * 50,
                (d, c) if c >= 3 => score += 100 * *d as u32 * 2u32.pow(*count as u32 - 3),
                _ => {} // Pairs of regular numbers
            }
        }
        score
    }

    /// Determines whether or not the player can roll again with this hand
    pub fn can_reroll(&self) -> bool {
        let counts = self.die_counts();

        match counts.keys().len() {
            6 => true,                                     // Straight 1 - 6
            3 if counts.values().all(|c| *c == 2) => true, // Three pairs
            2 if counts.values().all(|c| *c == 3) => true, // Two triplets
            _ => {
                // Triplet and any combination of fives and ones
                // Four of a kind and any combination of fives and ones
                // Five of a kind and a five or one

                let regulars: Vec<&Die> = counts.keys().filter(|d| **d != Die::One && **d != Die::Five).collect();
                // If the number of regulars is or exceeds 3, then the combination of regulars
                // scores and the player can re-roll
                if regulars.len() == 1 && (*counts.get(regulars[0]).unwrap() >= 3) {
                    return true;
                }
                false
            }
        }
    }

    /// Computes the difference between this roll and another. The diff contains dice that only
    /// appear in `other`.
    /// # Examples
    /// ```rust
    /// all_ones.diff(&straight) // [Die::Two, Die::Three, Die::Four, Die::Five, Die::Six]
    /// ```
    pub fn diff(&self, other: &Self) -> Vec<Die> {
        let mut difference: Vec<Die> = other.dice.into();
        for die in self.dice {
            if let Some(idx) = difference.iter().position(|x| die == *x) {
                difference.remove(idx);
            }
        }
        // Remove any don't cares since we can't actually roll them, and they can be any die
        // already existing in our current roll
        difference.into_iter().filter(|x| x != &Die::Any).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const RUN: [Die; 6] = [Die::One, Die::Two, Die::Three, Die::Four, Die::Five, Die::Six];
    const THREE_PAIRS: [Die; 6] = [Die::One, Die::Two, Die::One, Die::Two, Die::Five, Die::Five];

    mod score {
        use super::*;

        #[test]
        fn score_run() {
            let roll = Roll::new(RUN);
            assert_eq!(roll.score(), 2000);
        }

        #[test]
        fn score_three_pairs() {
            let roll = Roll::new(THREE_PAIRS);
            assert_eq!(roll.score(), 1500);
        }

        #[test]
        fn score_all_ones() {
            let roll = Roll::new([Die::One, Die::One, Die::One, Die::One, Die::One, Die::One]);
            assert_eq!(roll.score(), 8000);
        }

        #[test]
        fn score_three_threes() {
            let roll = Roll::new([Die::Three, Die::Two, Die::Three, Die::Three, Die::Two, Die::Four]);
            assert_eq!(roll.score(), 300);
        }

        #[test]
        fn score_three_fours_and_one() {
            let roll = Roll::new([Die::Four, Die::Two, Die::Four, Die::One, Die::Two, Die::Four]);
            assert_eq!(roll.score(), 500);
        }

        #[test]
        fn score_three_twos_and_five() {
            let roll = Roll::new([Die::Two, Die::Two, Die::Three, Die::Five, Die::Six, Die::Two]);
            assert_eq!(roll.score(), 250);
        }
    }

    mod reroll {
        use super::*;

        #[test]
        fn reroll_run() {
            let roll = Roll::new(RUN);
            assert!(roll.can_reroll());
        }

        #[test]
        fn cant_reroll_almost_run() {
            let roll = Roll::new([Die::One, Die::Two, Die::Two, Die::Four, Die::Five, Die::Six]);
            assert!(!roll.can_reroll());
        }

        #[test]
        fn reroll_three_pair() {
            let roll = Roll::new(THREE_PAIRS);
            assert!(roll.can_reroll());
        }

        #[test]
        fn reroll_five_of_a_kind_with_one() {
            let roll = Roll::new([Die::One, Die::Two, Die::Two, Die::Two, Die::Two, Die::Two]);
            assert!(roll.can_reroll());
        }

        #[test]
        fn reroll_four_of_a_kind_with_one_and_five() {
            let roll = Roll::new([Die::One, Die::Two, Die::Two, Die::Five, Die::Two, Die::Two]);
            assert!(roll.can_reroll());
        }

        #[test]
        fn reroll_two_triplets() {
            let roll = Roll::new([Die::Two, Die::Two, Die::Two, Die::Six, Die::Six, Die::Six]);
            assert!(roll.can_reroll());
        }

        #[test]
        fn reroll_three_of_a_kind_with_two_ones_and_five() {
            let roll = Roll::new([Die::Two, Die::Two, Die::Two, Die::One, Die::One, Die::Five]);
            assert!(roll.can_reroll());
        }

        #[test]
        fn cant_reroll_three_of_a_kind_with_two_ones_and_regular() {
            let roll = Roll::new([Die::Two, Die::Two, Die::Two, Die::One, Die::One, Die::Six]);
            assert!(!roll.can_reroll());
        }
    }

    mod difference {
        use crate::dice::{Die, Roll};

        #[test]
        fn compute_difference() {
            assert_eq!(
                Roll::new([Die::One, Die::One, Die::One, Die::One, Die::One, Die::One]).diff(&Roll::new([
                    Die::One,
                    Die::Two,
                    Die::One,
                    Die::Two,
                    Die::One,
                    Die::One,
                ])),
                vec![Die::Two, Die::Two],
            );
            assert_eq!(
                Roll::new([Die::One, Die::One, Die::One, Die::One, Die::One, Die::One]).diff(&Roll::new([
                    Die::One,
                    Die::Two,
                    Die::Three,
                    Die::Four,
                    Die::Five,
                    Die::Six,
                ])),
                vec![Die::Two, Die::Three, Die::Four, Die::Five, Die::Six],
            );
            assert_eq!(
                Roll::new([Die::One, Die::One, Die::Two, Die::Three, Die::Four, Die::Five]).diff(&Roll::new([
                    Die::One,
                    Die::One,
                    Die::One,
                    Die::Any,
                    Die::Any,
                    Die::Any,
                ])),
                vec![Die::One],
            );
        }
    }
}
