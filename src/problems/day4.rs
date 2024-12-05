const PATTERN: &str = "MAS";

/// Solve the problem for day four, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u32>, String> {
    let result_part_1 = count_all_xmas_seq(input_data);
    let result_part_2 = count_all_xmas_cross(input_data);

    Ok(vec![result_part_1, result_part_2])
}

/// Count all backwards and forwards instances of *XMAS*
fn count_all_xmas_seq(data: &[String]) -> u32 {
    let mut count = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            count += count_matches_at(i, j, data);
        }
    }

    count
}

/// Count all locations with a MAS cross
fn count_all_xmas_cross(data: &[String]) -> u32 {
    let mut count = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if has_cross_at(i, j, data) {
                count += 1;
            }
        }
    }

    count
}

/// Check for matches starting at a certain index
fn count_matches_at(i: usize, j: usize, data: &[String]) -> u32 {
    let char = get_char(data, i, j);
    let mut count = 0;
    if let Some(char) = char {
        if char == 'X' {
            count += find_sequence_match_at(i, j, 0, 1, data);
            count += find_sequence_match_at(i, j, 1, 1, data);
            count += find_sequence_match_at(i, j, 1, 0, data);
            count += find_sequence_match_at(i, j, 1, -1, data);
            count += find_sequence_match_at(i, j, 0, -1, data);
            count += find_sequence_match_at(i, j, -1, -1, data);
            count += find_sequence_match_at(i, j, -1, 0, data);
            count += find_sequence_match_at(i, j, -1, 1, data);
        }
    }

    count
}

/// Check if a location that contains X or S has a sequence that matches XMAS or SAMX.
fn find_sequence_match_at(
    i: usize,
    j: usize,
    increment_i: isize,
    increment_j: isize,
    data: &[String],
) -> u32 {
    for (index, c) in PATTERN.chars().enumerate() {
        let (i, j) = match increment_2d_index(i, j, increment_i, increment_j, index + 1) {
            Some(value) => value,
            None => return 0,
        };
        if !is_location_matching(i, j, data, c) {
            // The squence is broken, return
            return 0;
        }
    }

    1
}

/// Check if there is a cross at the location
fn has_cross_at(i: usize, j: usize, data: &[String]) -> bool {
    if is_location_matching(i, j, data, 'A') {
        return check_adjacent_for_cross(i, j, data);
    }

    false
}

/// Check both diagonals at the location for a valid M*S or S*M sequence.
fn check_adjacent_for_cross(i: usize, j: usize, data: &[String]) -> bool {
    check_diagonal(i, j, data, true) && check_diagonal(i, j, data, false)
}

/// Check the diagonals of a cross, to check is they match the M*S or S*M pattern
fn check_diagonal(i: usize, j: usize, data: &[String], up: bool) -> bool {
    let incr_i_1 = if up { -1 } else { 1 };
    let (i_1, j_1) = match increment_2d_index(i, j, incr_i_1, -1, 1) {
        Some(value) => value,
        None => return false,
    };

    let incr_i_2 = if up { 1 } else { -1 };
    let (i_2, j_2) = match increment_2d_index(i, j, incr_i_2, 1, 1) {
        Some(value) => value,
        None => return false,
    };

    if is_location_matching(i_1, j_1, data, 'M') {
        return is_location_matching(i_2, j_2, data, 'S');
    } else if is_location_matching(i_1, j_1, data, 'S') {
        return is_location_matching(i_2, j_2, data, 'M');
    }

    false
}

/// Check if the location at index i, j matches a certain char
fn is_location_matching(i: usize, j: usize, data: &[String], ref_char: char) -> bool {
    if let Some(char) = get_char(data, i, j) {
        if char == ref_char {
            return true;
        }
    }
    false
}

/// Get the character at a 2d index location from the slice of strings.
fn get_char(data: &[String], i: usize, j: usize) -> Option<char> {
    let row = data.get(i)?;
    let bytes = row.as_bytes();

    if !row.is_char_boundary(j) {
        return None;
    }

    let ch = std::str::from_utf8(&bytes[j..]).ok()?.chars().next()?;
    Some(ch)
}

/// Increment the i and j indexes with the increment. Return None if any is invalid.
fn increment_2d_index(
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
fn checked_add_increment(i: usize, increment: isize, factor: usize) -> Option<usize> {
    let incr = increment.checked_mul(factor as isize)?;
    if incr < 0 {
        i.checked_sub((-incr) as usize)
    } else {
        i.checked_add(incr as usize)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::solve;
    use crate::read_input_for_day;

    #[test]
    fn test_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("MMMSXXMASM").unwrap());
        data.push(String::from_str("MSAMXMSMSA").unwrap());
        data.push(String::from_str("AMXSXMAAMM").unwrap());
        data.push(String::from_str("MSAMASMSMX").unwrap());
        data.push(String::from_str("XMASAMXAMM").unwrap());
        data.push(String::from_str("XXAMMXXAMA").unwrap());
        data.push(String::from_str("SMSMSASXSS").unwrap());
        data.push(String::from_str("SAXAMASAAA").unwrap());
        data.push(String::from_str("MAMMMXMMMM").unwrap());
        data.push(String::from_str("MXMXAXMASX").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            18, result[0],
            "Result for part 1 example should be 18 but was {}",
            result[0]
        );
        assert_eq!(
            9, result[1],
            "Result for part 2 example should be 9 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_4() {
        let result =
            solve(&read_input_for_day(&4).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(2551, result[0]);
        assert_eq!(1985, result[1]);
    }
}