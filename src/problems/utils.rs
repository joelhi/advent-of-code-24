use std::{any, fs, io, str::FromStr};

/// Read the input for the specific day, given by the integer.
pub fn read_input_for_day(day: &i32) -> io::Result<Vec<String>> {
    let result = fs::read_to_string(format!("data/day{day}.txt"))?
        .lines()
        .map(String::from)
        .collect();

    Ok(result)
}

/// Parse a sequence of numbers in a string into a vec, assuming a single separator pattern.
pub fn parse_sequence_from_str<T: FromStr>(text: &str, separator: &str) -> Result<Vec<T>, String> {
    text.split(separator)
        .map(|s| {
            s.parse::<T>()
                .map_err(|_| format!("Failed to parse {} from {}", any::type_name::<T>(), s))
        })
        .collect()
}

/// Parse a pair of numbers from a str, assuming two values separated by some separator
pub fn parse_pair_from_str<T: FromStr + Copy>(
    text: &str,
    separator: &str,
) -> Result<(T, T), String> {
    let values = parse_sequence_from_str::<T>(text, separator)?;
    if values.len() != 2 {
        return Err(format!(
            "Expected only two values but found {}",
            values.len()
        ));
    }

    Ok((values[0], values[1]))
}
