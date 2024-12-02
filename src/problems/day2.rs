/// Solve the problem for day two, given the provided data.
pub fn solve(input_data: &Vec<String>) -> Result<Vec<u32>, String> {
    let input_sequences = parse_input_sequences(input_data)?;

    // Part 1
    let num_safe_sequences = input_sequences
        .iter()
        .filter(|sequence| validate_sequence(&sequence))
        .count()
        .try_into()
        .map_err(|_| "Value is too large to fit in u32")?;

    // Part 2


    Ok(vec![num_safe_sequences, 0])
}

// Parse each row of values into a vec
fn parse_input_sequences(input_data: &Vec<String>) -> Result<Vec<Vec<i32>>, String> {
    let mut all_sequences = Vec::with_capacity(input_data.len());
    for line in input_data.iter() {
        let values: Result<Vec<i32>, String> = line
            .split(" ")
            .map(|s| {
                s.parse::<i32>()
                    .map_err(|_| format!("Failed to convert '{}' to u32", s))
            })
            .collect();

        all_sequences.push(values?);
    }
    Ok(all_sequences)
}

// Validate a sequenc of numbers. Valid if all increasing or decreasing and max step <= 3.
fn validate_sequence(sequence: &[i32]) -> bool {
    let mut prev_diff: i32 = 0;
    for i in 1..sequence.len() {
        let diff = sequence[i-1] - (sequence[i]);
        if diff.abs() > 3 || diff.abs() < 1 || (i > 1 && prev_diff.signum() != diff.signum()){
            return false;
        }
        prev_diff = diff;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::{day2::solve, read_input_for_day};

    #[test]
    fn test_day_2() {
        let result =
            solve(&read_input_for_day(&2).expect("Expect the data file to be there.")).unwrap();
        
        assert_eq!(242, result[0]);
    }
}