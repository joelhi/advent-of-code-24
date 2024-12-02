use std::collections::HashMap;

/// Solve the problem for day one, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u32>, String> {
    let (mut first_list, mut second_list) = read_input_into_lists(input_data)?;

    // Part 1
    first_list.sort();
    second_list.sort();

    let mut answer_part_1 = 0;
    for (first, second) in first_list.iter().zip(&second_list) {
        answer_part_1 += first.abs_diff(*second);
    }

    // Part 2
    let mut matches: HashMap<u32, u32> = HashMap::with_capacity(first_list.len());
    for &num in second_list.iter() {
        *matches.entry(num).or_insert(0) += 1;
    }

    let mut answer_part_2 = 0;
    for &num in first_list.iter() {
        let num_matches = *matches.get(&num).unwrap_or(&0);
        answer_part_2 += num * num_matches;
    }

    Ok(vec![answer_part_1, answer_part_2])
}

// Parse the text data into two lists of u32s.
fn read_input_into_lists(data: &[String]) -> Result<(Vec<u32>, Vec<u32>), String> {
    let mut first_list: Vec<u32> = Vec::with_capacity(data.len());
    let mut second_list: Vec<u32> = Vec::with_capacity(data.len());

    for s in data.iter() {
        let (first_val, second_val) = split_entry(s)?;
        first_list.push(first_val);
        second_list.push(second_val);
    }

    Ok((first_list, second_list))
}

// Split on entry into two values
fn split_entry(s: &str) -> Result<(u32, u32), String> {
    let parts: Vec<&str> = s.split("   ").collect();

    if parts.len() != 2 {
        return Err(format!("Failed to split data entry {} properly.", s));
    }

    let first_value = parts[0]
        .parse::<u32>()
        .or(Err(format!("Failed to parse {} into u32.", parts[0])))?;

    let second_value = parts[1]
        .parse::<u32>()
        .or(Err(format!("Failed to parse {} into u32.", parts[1])))?;

    Ok((first_value, second_value))
}
#[cfg(test)]
mod tests {
    use crate::{day1::solve, read_input_for_day};

    #[test]
    fn test_day_1() {
        let result =
            solve(&read_input_for_day(&1).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(2367773, result[0]);
        assert_eq!(21271939, result[1])
    }
}
