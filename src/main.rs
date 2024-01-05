use std::env;
mod dice;

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

    Ok(())
}
