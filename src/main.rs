mod combos;
mod dice;
use std::env;

use crate::combos::chances;

fn usage(prog: &str) {
    eprintln!("{prog} dice");
    eprintln!("dice\t\tThe face values of the dice in the current role, separated by spaces.");
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let current_roll: dice::Roll = match args[1..].try_into() {
        Ok(r) => r,
        Err(dice::RollConstructionError::WrongDieCount) => {
            usage(&args[0]);
            return Err("Expected 6 die faces as arguments.".to_string());
        }
        Err(dice::RollConstructionError::InvalidDie(e)) => {
            usage(&args[0]);
            return Err(format!("{e}"));
        }
    };

    println!("Your roll is worth {} points.", current_roll.score());
    if current_roll.can_reroll() {
        println!("You can roll again! Re-roll all your dice.");
        return Ok(()); // Early return since we're just going to re-roll all dice no matter what
    }

    // The only rolls that should be computed are rolls that provide an increase in score
    // Out of these rolls, show only the most probable variant among all rolls with the same score
    // Example:
    // Given roll: (1 1) 2 3 4 5
    // If we gain another 1 we can increase our score from 200 to 1000
    // Desired roll: (1 1 1) x x x (x values don't matter)
    // Highest probability to achieve this variant is rolling 2, 3, 4 and 5 again.
    // Do NOT show variant where you re-roll one of the 1s as well and hope to get 2 1s.
    let better_roll = dice::Roll::new([
        dice::Die::One,
        dice::Die::One,
        dice::Die::One,
        dice::Die::Any,
        dice::Die::Any,
        dice::Die::Any,
    ]);

    println!("Chances: {}", current_roll.chances_to(&better_roll));

    Ok(())
}
