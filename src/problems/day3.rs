use regex::Regex;

const PATTERN: &'static str = r"mul\(\d+,\d+\)";

/// Solve the problem for day three, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u32>, String> {
    // Part 1
    let valid_operations = find_valid_operations(input_data);
    let result = execute_operations(&valid_operations)?;

    // Part 2

    Ok(vec![result, 0])
}

// Filter the input data and find all valid operations
fn find_valid_operations(input_data: &[String]) -> Vec<&str> {
    let re = Regex::new(PATTERN).expect("Should be able to create a regex from the pattern.");
    let mut all_ops = Vec::new();
    for s in input_data {
        all_ops.extend(re.find_iter(s).map(|m| m.as_str()));
    }

    all_ops
}

// Execute a list of operations defined as &str
fn execute_operations(ops: &[&str]) -> Result<u32, String> {
    let mut result = 0;
    for operation in ops {
        result += execute_single_from_string(operation)?;
    }
    Ok(result)
}

// Parse a single operation from text format into the values and compute the result
fn execute_single_from_string(operation: &str) -> Result<u32, String> {
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
        .parse::<u32>()
        .map_err(|_| format!("Failed to parse {} as u32.", split[0]))?;

    let second_number = &split[1][..split[1].len() - 1]
        .parse::<u32>()
        .map_err(|_| format!("Failed to parse {} as u32.", split[0]))?;

    Ok(first_number * second_number)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::read_input_for_day;

    #[test]
    fn test_day_3() {
        let result =
            solve(&read_input_for_day(&3).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(173529487, result[0]);
    }
}
