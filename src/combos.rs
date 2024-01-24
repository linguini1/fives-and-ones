use crate::dice::Die;

/// Calculates the chance of rolling the given set of dice in `rolls` number of rolls with `ndice` dice.
///
/// # Arguments
///
/// * `desired` - The desired set of dice faces to roll.
/// * `ndice` - The number of dice that will be rolled to achieve the desired set of dice.
///
/// # Returns
/// The chances of rolling the desired set of dice using `ndice` dice expressed as a decimal fraction of 1.
///
/// # Examples
/// ```rust
/// chances(&[Die::One], 1); // 0.1666...
/// chances(&[Die::Three], 2); // 0.305555...
/// chances(&[Die::One, Die::One], 6); // 0.200938786
/// ```
pub fn chances(desired: &[Die], ndice: u8) -> f32 {
    if desired.len() > ndice as usize {
        return 0.0;
    }

    let _total_outcomes = 6u32.pow(ndice.into());

    let mut chance: f32 = 0.0;
    for k in desired.len() as u8..ndice + 1 {
        chance += binomial(ndice.into(), k as u64, 1.0 / 6.0)
    }
    chance
}

/// Calculate the factorial of a number.
fn factorial(mut number: u64) -> u64 {
    let mut total: u64 = 1;
    while number > 0 {
        total *= number;
        number -= 1;
    }
    total
}

/// Calculate the result of n choose m.
fn choose(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

/// Calculate binomial probability.
fn binomial(n: u64, k: u64, p: f32) -> f32 {
    choose(n, k) as f32 * p.powf(k as f32) * (1.0 - p).powf((n - k) as f32)
}

#[cfg(test)]
mod tests {

    const DELTA: f32 = 0.0001;
    macro_rules! assert_approx {
        ($actual:expr, $expected:expr, $delta:expr) => {
            assert!($expected - $delta <= $actual && $actual <= $expected + $delta);
        };
    }

    mod mathematics {
        use super::DELTA;
        use crate::combos::{binomial, choose, factorial};

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
            assert_eq!(choose(6, 2), 15);
            assert_eq!(choose(2, 1), 2);
        }

        #[test]
        fn binomials() {
            assert_approx!(binomial(6, 2, 1.0 / 6.0), 0.200_938_79_f32, DELTA);
        }
    }

    mod combinatorics {
        use super::DELTA;
        use crate::combos::chances;
        use crate::dice::Die;

        #[test]
        fn not_enough_dice() {
            assert_approx!(chances(&[Die::One, Die::Two, Die::Three], 2), 0.0, DELTA);
        }

        #[test]
        fn single_die_single_roll() {
            assert_approx!(chances(&[Die::One], 1), 1.0 / 6.0, DELTA);
        }

        #[test]
        fn one_desired() {
            assert_approx!(chances(&[Die::One], 6), 0.6651012, DELTA);
        }

        #[test]
        fn two_desired() {
            assert_approx!(chances(&[Die::One, Die::One], 6), 0.263_224_45_f32, DELTA);
        }

        #[test]
        fn two_dice() {
            assert_approx!(chances(&[Die::Three], 2), 0.305_555_55_f32, DELTA);
        }
    }
}
