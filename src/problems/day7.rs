use super::utils;

const BASE: usize = 2;

/// Solve the problem for day six, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let operations = parse_inputs(input_data)?;

    let result_part_1 = operations
        .iter()
        .filter(|op| validate_operation(op).is_ok_and(|b| b))
        .map(|(r, _)| *r)
        .sum();

    Ok(vec![result_part_1, 0])
}

/// Check if a solution exists to return the correct result
fn validate_operation(operation: &(u64, Vec<u64>)) -> Result<bool, String> {
    let (result, inputs) = operation;
    let n = inputs.len();
    let max = BASE.pow(n.try_into().map_err(|_| format!("Cannot convert {} to u32", n))?);
    for count in 0..max {
        let operations = match generate_binary_sequence(n, count) {
            Some(sequence) => Ok(sequence),
            None => Err("Failed to generate sequence of operations".to_owned()),
        }?;

        if *result == execute_operation(inputs, &operations)? {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Execute a sequence of inputs with a sequence of operations, either [`x`] or [`+`]
fn execute_operation(inputs: &[u64], operation_sequence: &[char]) -> Result<u64, String> {
    let mut result = 0;
    for (val, op) in inputs.iter().zip(operation_sequence) {
        result = match op {
            'x' => Ok(result * val),
            '+' => Ok(result + val),
            _ => Err("".to_owned()),
        }?
    }

    Ok(result)
}

/// Represent a number as a binary sequence of operations [`x`] or [`+`].
fn generate_binary_sequence(n: usize, count: usize) -> Option<Vec<char>> {
    if count >= (1 << n + 1) {
        return None;
    }

    Some(
        (0..n)
            .rev()
            .map(|i| if (count & (1 << i)) != 0 { 'x' } else { '+' })
            .collect(),
    )
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

    use super::solve;
    use crate::read_input_for_day;

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
            "Result for part 1 example should be 3 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_7() {
        let result =
            solve(&read_input_for_day(&7).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(3351424677624, result[0]);
        assert_eq!(0, result[1]);
    }
}
