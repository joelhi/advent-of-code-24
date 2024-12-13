use regex::Regex;

const OPERATION_PATTERN: &str = r"mul\(\d+,\d+\)";
const DO_PATTERN: &str = r"do\(\)";
const DONT_PATTERN: &str = r"don't\(\)";

/// Solve the problem for day three, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    // Merge rows into single string
    let full_data = input_data.join("");

    // Part 1
    let valid_operations = find_valid_operations(&full_data);
    let result_part_1 = execute_all_operations(&valid_operations)?;

    // Part 2
    let operations_and_triggers = find_operations_and_triggers(&full_data);
    let result_part_2 = execute_enabled_operations(&operations_and_triggers)?;

    Ok(vec![result_part_1, result_part_2])
}

// Filter the input data and find all valid operations
fn find_valid_operations(input_data: &str) -> Vec<&str> {
    let re =
        Regex::new(OPERATION_PATTERN).expect("Should be able to create a regex from the pattern.");
    let mut all_ops = Vec::new();
    all_ops.extend(re.find_iter(input_data).map(|m| m.as_str()));

    all_ops
}

// Execute a list of operations defined as &str
fn execute_all_operations(ops: &[&str]) -> Result<u64, String> {
    let mut result = 0;
    for operation in ops {
        result += execute_single_from_string(operation)?;
    }
    Ok(result)
}

// Execute the enabled instruction based on the embedded trigger instructions
fn execute_enabled_operations(instructions: &[&str]) -> Result<u64, String> {
    let mut result = 0;
    let mut enabled = true;
    for &instruction in instructions {
        match instruction {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            &_ => {
                if enabled {
                    result += execute_single_from_string(instruction)?;
                }
            }
        }
    }
    Ok(result)
}

// Parse a single operation from text format into the values and compute the result
fn execute_single_from_string(operation: &str) -> Result<u64, String> {
    // Extract the parenthesis bit
    let operation_values = &operation[3..];
    let split: Vec<&str> = operation_values.split(",").collect();

    if split.len() != 2 {
        return Err(format!(
            "Failed to parse operation values from {}",
            operation_values
        ));
    }

    // Parse the strings into integers
    let first_number = split[0][1..]
        .parse::<u64>()
        .map_err(|_| format!("Failed to parse {} as u32.", split[0]))?;

    let second_number = &split[1][..split[1].len() - 1]
        .parse::<u64>()
        .map_err(|_| format!("Failed to parse {} as u32.", split[0]))?;

    Ok(first_number * second_number)
}

// Find all the matches for either operations or do / don't triggers
fn find_operations_and_triggers(input_data: &str) -> Vec<&str> {
    // Define all relevant regex patterns
    let main_pattern =
        Regex::new(OPERATION_PATTERN).expect("Should be able to create the regex pattern.");
    let do_pattern = Regex::new(DO_PATTERN).expect("Should be able to create the regex pattern.");
    let dont_pattern =
        Regex::new(DONT_PATTERN).expect("Should be able to create the regex pattern.");

    // Collect all matches
    let mut all_matches: Vec<(usize, &str)> = Vec::new();

    for mat in main_pattern.find_iter(input_data) {
        all_matches.push((mat.start(), mat.as_str()));
    }
    for mat in do_pattern.find_iter(input_data) {
        all_matches.push((mat.start(), mat.as_str()));
    }
    for mat in dont_pattern.find_iter(input_data) {
        all_matches.push((mat.start(), mat.as_str()));
    }

    // Sort the matches by their start position
    all_matches.sort_by_key(|&(pos, _)| pos);

    all_matches.iter().map(|item| item.1).collect()
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::read_input_for_day;

    #[test]
    fn test_day_3() {
        let result =
            solve(&read_input_for_day(3).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(173529487, result[0]);
        assert_eq!(99532691, result[1]);
    }
}
