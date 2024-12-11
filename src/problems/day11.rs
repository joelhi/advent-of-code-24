use std::collections::{HashMap, HashSet};

use super::utils;

/// Solve the problem for day eleven, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let mut stone_map = parse_stone_map(&input_data[0])?;

    for _ in 0..25 {
        map_stones(&mut stone_map)?;
    }

    Ok(vec![])
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

fn map_stones(stone_map: &mut HashMap<u64, u64>) -> Result<(), String> {
    let keys: HashSet<u64> = stone_map.keys().copied().collect();
    for val in keys {
        if val == 0 {
            if let Some(&count) = stone_map.get(&0) {
                stone_map.remove(&0);
                *stone_map.entry(1).or_insert(0) += count;
            }
        } else if let Some((a, b)) = split_val(val) {
            if let Some(&count) = stone_map.get(&val) {
                stone_map.remove(&val);
                *stone_map.entry(a).or_insert(0) += count;
                *stone_map.entry(b).or_insert(0) += count;
            }
        } else {
            if let Some(&count) = stone_map.get(&val) {
                stone_map.remove(&val);
                *stone_map.entry(val * 2024).or_insert(0) += count;
            }
        }
    }

    Ok(())
}

fn split_val(val: u64) -> Option<(u64, u64)> {
    let num_str = val.to_string();
    let len = num_str.len();

    if !len % 2 == 0 {
        return None;
    }

    let mid = len / 2;

    // Split the string into two halves
    let (first_half, second_half) = num_str.split_at(mid);

    // Convert the halves back to u64
    let first_half_num = first_half.parse::<u64>().unwrap_or(0);
    let second_half_num = second_half.parse::<u64>().unwrap_or(0);

    Some((first_half_num, second_half_num))
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
            "Result for part 1 example should be 36 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_11() {
        let result =
            solve(&read_input_for_day(&11).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(0, result[0]);
        assert_eq!(0, result[1]);
    }
}
