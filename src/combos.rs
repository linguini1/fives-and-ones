use crate::dice::Die;
use crate::dice::DICE_COUNT;

/// Calculate the chance of rolling the given set of dice in `rolls` number of rolls.
pub fn chances(dice: Vec<Die>, rolls: u16) -> f32 {
    // Chances of not rolling the specific die
    let mut chance_no_roll: f32 = 1.0;
    for _die in dice {
        chance_no_roll *= 5.0 / 6.0;
    }

    let mut chance: f32 = chance_no_roll;
    for _ in 1..rolls {
        chance *= chance_no_roll;
    }

    1.0 - chance
}

/// Calculate the factorial of a number
fn factorial(number: u64) -> u64 {
    let mut total: u64 = 1;
    let mut n = number;
    while n > 0 {
        total *= n;
        n -= 1;
    }
    total
}

#[cfg(test)]
mod tests {

    mod fact {
        use crate::combos::factorial;

        #[test]
        fn factorial_0() {
            assert_eq!(factorial(0), 1);
        }

        #[test]
        fn factorial_1() {
            assert_eq!(factorial(1), 1);
        }

        #[test]
        fn factorial_6() {
            assert_eq!(factorial(6), 720);
        }

        #[test]
        fn factorial_large_number() {
            assert_eq!(factorial(12), 479001600);
        }
    }

    mod combinatorics {
        use crate::combos::chances;
        use crate::combos::factorial;
        use crate::dice::Die;

        const DELTA: f32 = 0.001;
        const ONE_IN_SIX: f32 = 1.0 / 6.0;

        macro_rules! assert_approx {
            ($actual:expr, $expected:expr, $delta:expr) => {
                assert!($expected - $delta <= $actual && $actual <= $expected + $delta);
            };
        }

        #[test]
        fn single_die() {
            assert_approx!(chances(vec![Die::One], 1), ONE_IN_SIX, DELTA);
        }

        #[test]
        fn two_dice() {
            assert_approx!(
                chances(vec![Die::One, Die::Two], 1),
                factorial(6) as f32 / (factorial(2) as f32 * (6.0 - 2.0)),
                DELTA
            );
        }
    }
}
