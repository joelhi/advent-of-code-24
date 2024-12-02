pub mod problems;

use problems::utils::*;
use problems::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let day = &args[1].parse::<i32>().unwrap();

    println!("Running day {}", day);
    match solve_day(day) {
        Ok(result) => {
            println!("The answers are:");
            for (i, val) in result.iter().enumerate() {
                println!("{}: {}", i, val);
            }
        }
        Err(err) => println!("Failed with error: {}", err),
    }
}

fn solve_day(day: &i32) -> Result<Vec<u32>, String> {
    let input_data =
        read_input_for_day(day).or(Err(format!("Failed to read input data for day {}", day)))?;

    match day {
        1 => day1::solve(&input_data),
        2 => day2::solve(&input_data),
        ..=25 => Err(format!("Day {} is not solved yet", day)),
        _ => Err(format!(
            "Number {} is not a valid day for the calendar...",
            day
        )),
    }
}
