use std::env;
mod die;

fn usage(prog: &str) {
    eprintln!("{prog} dice");
    eprintln!("dice\t\tThe face values of the dice in the current role, separated by spaces.");
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let current_roll: die::Roll = match args[1..].try_into() {
        Ok(r) => r,
        Err(die::RollConstructionError::WrongDieCount) => {
            usage(&args[0]);
            return Err("Expected 6 die faces as arguments.".to_string());
        }
        Err(die::RollConstructionError::InvalidDie(e)) => {
            usage(&args[0]);
            return Err(format!("{e}"));
        }
    };

    println!("{current_roll}");

    Ok(())
}
