use crate::dice::Die;

/// Calculate the chance of rolling the given set of dice in `rolls` number of rolls.
pub fn chances(dice: &[Die], rolls: u16) -> f32 {
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
fn factorial(mut number: u64) -> u64 {
    let mut total: u64 = 1;
    while number > 0 {
        total *= number;
        number -= 1;
    }
    total
}

/// Calculate the result of n choose m
fn choose(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

#[cfg(test)]
mod tests {

    mod mathematics {
        use crate::combos::{choose, factorial};

        #[test]
        fn factorials() {
            assert_eq!(factorial(0), 1);
            assert_eq!(factorial(1), 1);
            assert_eq!(factorial(6), 720);
            assert_eq!(factorial(12), 479001600);
        }

        #[test]
        fn choosing() {
            assert_eq!(choose(12, 2), 66);
            assert_eq!(choose(2, 2), 1);
            assert_eq!(choose(2, 1), 2);
        }
    }

    mod combinatorics {
        use crate::combos::{chances, factorial};
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
            assert_approx!(chances(&[Die::One], 1), ONE_IN_SIX, DELTA);
        }

        #[test]
        fn two_dice() {
            assert_approx!(
                chances(&[Die::One, Die::Two], 1),
                factorial(6) as f32 / (factorial(2) as f32 * 4.0),
                DELTA
            );
        }
    }
}
