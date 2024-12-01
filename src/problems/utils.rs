use std::{fs, io};

/// Read the input for the specific day, given by the integer.
pub fn read_input_for_day(day: &i32) -> io::Result<Vec<String>> {
    let result = fs::read_to_string(format!("data/day{day}.txt"))?
        .lines()
        .map(String::from)
        .collect();

    Ok(result)
}
