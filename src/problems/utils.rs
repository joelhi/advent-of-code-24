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

/// Get the character at a 2d index location from the slice of strings.
pub fn get_char(data: &[String], i: usize, j: usize) -> Option<char> {
    let row = data.get(i)?;
    let bytes = row.as_bytes();

    if !row.is_char_boundary(j) {
        return None;
    }

    let ch = std::str::from_utf8(&bytes[j..]).ok()?.chars().next()?;
    Some(ch)
}

/// Increment the i and j indexes with the increment. Return None if any is invalid.
pub fn increment_2d_index(
    i: usize,
    j: usize,
    increment_i: isize,
    increment_j: isize,
    factor: usize,
) -> Option<(usize, usize)> {
    let i = checked_add_increment(i, increment_i, factor)?;
    let j = checked_add_increment(j, increment_j, factor)?;

    Some((i, j))
}

/// Increment or decrement the unsigned index
pub fn checked_add_increment(i: usize, increment: isize, factor: usize) -> Option<usize> {
    let incr = increment.checked_mul(factor as isize)?;
    if incr < 0 {
        i.checked_sub((-incr) as usize)
    } else {
        i.checked_add(incr as usize)
    }
}

/// Transposes a grid of chars represented as a list of Strings.
pub fn transpose_text_data(data: &[String]) -> Result<Vec<String>, String> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let num_cols = data[0].len();
    let mut transposed = Vec::with_capacity(num_cols);

    for i in 0..num_cols {
        let mut new_row = String::with_capacity(data.len());
        for j in 0..data.len() {
            new_row.push(
                get_char(data, i, j)
                    .ok_or(format!("Failed to get char from string at {},{}", i, j))?,
            );
        }
        transposed.push(new_row);
    }

    Ok(transposed)
}
