pub mod problems;

use std::time::Instant;

use problems::utils::*;
use problems::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let day = &args[1].parse::<i32>().unwrap();
    let before = Instant::now();
    println!("Running day {}", day);
    match solve_day(*day) {
        Ok(result) => {
            println!("The answers are:");
            for (i, val) in result.iter().enumerate() {
                println!("{}: {}", i, val);
            }
            println!("Solution completed in {:2}ms", before.elapsed().as_millis())
        }
        Err(err) => println!("Failed with error: {}", err),
    }
}

fn solve_day(day: i32) -> Result<Vec<u64>, String> {
    let input_data =
        read_input_for_day(day).or(Err(format!("Failed to read input data for day {}", day)))?;

    match day {
        1 => day1::solve(&input_data),
        2 => day2::solve(&input_data),
        3 => day3::solve(&input_data),
        4 => day4::solve(&input_data),
        5 => day5::solve(&input_data),
        6 => day6::solve(&input_data),
        7 => day7::solve(&input_data),
        8 => day8::solve(&input_data),
        9 => day9::solve(&input_data),
        10 => day10::solve(&input_data),
        11 => day11::solve(&input_data),
        12 => day12::solve(&input_data),
        ..=25 => Err(format!("Day {} is not solved yet", day)),
        _ => Err(format!(
            "Number {} is not a valid day for the calendar...",
            day
        )),
    }
}
