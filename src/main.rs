use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Usage: <roll_string>\n\
            Example: 2d6+1d4+3"
        );
        return;
    }
    let roll_string = &args[1];
    let result = parse_and_roll(roll_string);
    match result {
        Ok(total) => println!("Total roll: {}", total),
        Err(e) => println!("Error: {}", e),
    }
}

/// Parses a roll instruction, rolls, and returns the result.
#[inline]
fn parse_and_roll(roll_string: &str) -> Result<u32, &'static str> {
    let parts = roll_string.split('+');
    let mut total: u32 = 0;
    for part in parts {
        if part.contains('d') {
            let dice_parts: Vec<&str> = part.split('d').collect();
            if dice_parts.len() != 2 {
                return Err("Invalid roll format");
            }
            let dice_count: u32 = dice_parts[0].parse().map_err(|_| "Invalid number of dice")?;
            let dice_type: u32 = dice_parts[1].parse().map_err(|_| "Invalid dice type")?;
            for _ in 0..dice_count {
                total += roll_dice(dice_type);
            }
        } else {
            let modifier: u32 = part.parse().map_err(|_| "Invalid modifier")?;
            total += modifier;
        }
    }
    Ok(total)
}

/// Rolls a die with a specific number of `sides`.
#[inline]
#[must_use]
fn roll_dice(sides: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=sides)
}