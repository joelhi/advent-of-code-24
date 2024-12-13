use std::collections::HashMap;

use super::utils;

/// Solve the problem for day eleven, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let mut stone_map = parse_stone_map(&input_data[0])?;

    // Part 1
    for _ in 0..25 {
        map_stones(&mut stone_map)?;
    }
    let result_part_1 = stone_map.values().sum();

    // Continue for part 2
    for _ in 0..50 {
        map_stones(&mut stone_map)?;
    }
    let result_part_2 = stone_map.values().sum();

    Ok(vec![result_part_1, result_part_2])
}

/// Maps the initial stones into a map, with the count of each number
fn parse_stone_map(input: &str) -> Result<HashMap<u64, u64>, String> {
    let all_stones = utils::parse_sequence_from_str::<u64>(input, " ")?;
    let mut unique_stones = HashMap::with_capacity(all_stones.len());
    for val in all_stones {
        *unique_stones.entry(val).or_insert(0) += 1;
    }

    Ok(unique_stones)
}

/// Compute a new map from the stones in the input map, by applying the rules
fn map_stones(stone_map: &mut HashMap<u64, u64>) -> Result<(), String> {
    let mut new_map = HashMap::with_capacity(stone_map.len());

    for (&val, &count) in stone_map.iter() {
        if val == 0 {
            *new_map.entry(1).or_insert(0) += count;
        } else if let Some((a, b)) = split_val(val)? {
            *new_map.entry(a).or_insert(0) += count;
            *new_map.entry(b).or_insert(0) += count;
        } else {
            *new_map.entry(val * 2024).or_insert(0) += count;
        }
    }
    *stone_map = new_map;

    Ok(())
}

// Split a val with even number of digits into two numbers.
fn split_val(val: u64) -> Result<Option<(u64, u64)>, String> {
    let num_str = val.to_string();
    let len = num_str.len();

    if !len % 2 == 0 {
        return Ok(None);
    }

    let (first_half, second_half) = num_str.split_at(len / 2);
    let first_half_num = first_half
        .parse::<u64>()
        .map_err(|_| format!("Failed to parse {} into a number", first_half))?;
    let second_half_num = second_half
        .parse::<u64>()
        .map_err(|_| format!("Failed to parse {} into a number", second_half))?;

    Ok(Some((first_half_num, second_half_num)))
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
        data.push(String::from_str("125 17").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            55312, result[0],
            "Result for part 1 example should be 55312 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_11() {
        let result =
            solve(&read_input_for_day(11).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(229043, result[0]);
        assert_eq!(272673043446478, result[1]);
    }
}
