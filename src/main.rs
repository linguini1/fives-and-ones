use std::env;
mod die;

fn usage(prog: &str) {
    eprintln!("{prog} dice");
    eprintln!("dice\t\tThe face values of the dice in the current role, separated by spaces.");
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        usage(&args[0]);
        return Err("All 6 die faces must be provided.".to_string());
    }

    let mut current_roll: Vec<die::Die> = vec![];
    for n in args[1..].iter() {
        current_roll.push(match n.clone().try_into() {
            Ok(d) => d,
            Err(die::DieConstructionError::NonDigit) => return Err(format!("Input {n} is not a digit.")),
            Err(die::DieConstructionError::OutOfRange) => return Err(format!("Digit {n} is not a die face from 1-6.")),
        });
    }

    println!("{current_roll:?}");

    Ok(())
}
