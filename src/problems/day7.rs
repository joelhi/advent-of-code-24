use super::utils;

/// Solve the problem for day seven, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let equations = parse_inputs(input_data)?;

    let result_part_1 = equations
        .iter()
        .filter(|eq| validate_equation(eq, &['x', '+']).is_ok_and(|b| b))
        .map(|(r, _)| *r)
        .sum();

    let result_part_2 = equations
        .iter()
        .filter(|eq| validate_equation(eq, &['x', '+', '|']).is_ok_and(|b| b))
        .map(|(r, _)| *r)
        .sum();

    Ok(vec![result_part_1, result_part_2])
}

/// Check if a solution exists to return the correct result
fn validate_equation(equation: &(u64, Vec<u64>), operations: &[char]) -> Result<bool, String> {
    let (result, inputs) = equation;
    dfs(result, inputs[0], inputs, operations, 0)
}

/// Run a Depth-First search on the possible combinations.
fn dfs(
    result: &u64,
    value: u64,
    inputs: &[u64],
    operations: &[char],
    depth: usize,
) -> Result<bool, String> {
    // Reached end of tree, check final value and target.
    if depth == inputs.len() - 1 {
        return Ok(value == *result);
    }

    // Not at end but value to0 large, abort branch
    if value > *result {
        return Ok(false);
    }

    // Evaluate next step starting from current node.
    for op in operations {
        let new_val = execute_operation(&value, &inputs[depth + 1], op)?;
        if dfs(result, new_val, inputs, operations, depth + 1)? {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Execute a single operation on two values, either [`x`], [`+`] or [`|`]
fn execute_operation(lhs: &u64, rhs: &u64, operation: &char) -> Result<u64, String> {
    match operation {
        'x' => Ok(lhs * rhs),
        '+' => Ok(lhs + rhs),
        '|' => concat_values(lhs, rhs),
        _ => Err("Invalid operation".to_owned()),
    }
}

/// Concatenate two values into a new value
fn concat_values(lhs: &u64, rhs: &u64) -> Result<u64, String> {
    let mut s = lhs.to_string();
    s.push_str(&rhs.to_string());
    s.parse::<u64>()
        .map_err(|_| format!("Failed to parse {} into u64", s))
}

/// Parse the input data into the result and the inputs
fn parse_inputs(input_data: &[String]) -> Result<Vec<(u64, Vec<u64>)>, String> {
    input_data.iter().map(|s| parse_row(s)).collect()
}

/// Parse a single row into result and inputs.
fn parse_row(row: &str) -> Result<(u64, Vec<u64>), String> {
    let parts: Vec<&str> = row.split(':').collect();

    if parts.len() != 2 {
        return Err(format!("Failed to split {} into result and inputs.", row));
    }

    let result = parts[0]
        .parse::<u64>()
        .map_err(|_| format!("Cannot parse u64 from {}", parts[0]))?;

    let inputs = utils::parse_sequence_from_str::<u64>(&parts[1][1..], " ")?;

    Ok((result, inputs))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("190: 10 19").unwrap());
        data.push(String::from_str("3267: 81 40 27").unwrap());
        data.push(String::from_str("83: 17 5").unwrap());
        data.push(String::from_str("156: 15 6").unwrap());
        data.push(String::from_str("7290: 6 8 6 15").unwrap());
        data.push(String::from_str("161011: 16 10 13").unwrap());
        data.push(String::from_str("192: 17 8 14").unwrap());
        data.push(String::from_str("21037: 9 7 18 13").unwrap());
        data.push(String::from_str("292: 11 6 16 20").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            3749, result[0],
            "Result for part 1 example should be 3749 but was {}",
            result[0]
        );
        assert_eq!(
            11387, result[1],
            "Result for part 2 example should be 11387 but was {}",
            result[0]
        )
    }

    #[test]
    #[cfg(feature = "real_inputs")]
    fn test_day_7() {
        let result = solve(&crate::read_input_for_day(7).expect(
            "To run the tests for the real inputs the file has to be found in the inputs folder.",
        ))
        .unwrap();

        assert_eq!(3351424677624, result[0]);
        assert_eq!(204976636995111, result[1]);
    }
}
