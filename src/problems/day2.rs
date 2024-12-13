use crate::parse_sequence_from_str;

// Alias for a difference between two values, given as the index and the size.
type DiffTuple = (usize, i32);

/// Solve the problem for day two, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    // Parse inputs
    let input_sequences = input_data
        .iter()
        .map(|line| {
            parse_sequence_from_str(line, " ")
                .map_err(|_| format!("Failed to parse {} into integers", line))
        })
        .collect::<Result<Vec<Vec<i32>>, String>>()?;

    // Part 1
    let num_safe_sequences: u64 = input_sequences
        .iter()
        .filter(|sequence| validate_sequence(sequence))
        .count()
        .try_into()
        .map_err(|_| "Value is too large to fit in u32")?;

    // Part 2
    let num_safe_sequences_with_dampening: u64 = input_sequences
        .iter()
        .filter(|sequence| validate_sequence_with_dampening(sequence))
        .count()
        .try_into()
        .map_err(|_| "Value is too large to fit in u32")?;

    Ok(vec![num_safe_sequences, num_safe_sequences_with_dampening])
}

// Validate a sequence of numbers. Valid if all increasing or decreasing and max step <= 3.
fn validate_sequence(sequence: &[i32]) -> bool {
    let mut prev_diff: i32 = 0;
    for i in 1..sequence.len() {
        let diff = sequence[i - 1] - (sequence[i]);
        if is_diff_unsafe(i, diff, prev_diff) {
            return false;
        }
        prev_diff = diff;
    }

    true
}

// Check if the diff can be considered unsafe
fn is_diff_unsafe(i: usize, diff: i32, prev_diff: i32) -> bool {
    !(1..4).contains(&diff.abs()) || (i > 1 && prev_diff.signum() != diff.signum())
}

// Validate a sequence of numbers. Valid if all increasing or decreasing and max step <= 3, allowing one bad number in a sequence.
fn validate_sequence_with_dampening(sequence: &[i32]) -> bool {
    let diffs: Vec<i32> = (1..sequence.len())
        .map(|i| sequence[i] - sequence[i - 1])
        .collect();

    // Validate diffs
    let problematic = get_problematic_diffs(&diffs);

    if problematic.is_empty() {
        return true;
    } else if problematic.len() < 3 {
        // If the sequence has diffs causing problems but is potentally solveable, try to remove the entry before or after.
        let mut valid_found = false;
        for problematic_diff in problematic {
            if validate_sequence_with_removed(sequence, problematic_diff.0)
                || validate_sequence_with_removed(sequence, problematic_diff.0 + 1)
            {
                valid_found = true;
                break;
            }
        }
        return valid_found;
    }

    false
}

// Find the diffs that are causing issues
fn get_problematic_diffs(diffs: &[i32]) -> Vec<DiffTuple> {
    let (increases, decreases, invalid) = sort_increment_types(diffs);

    let mut problematic = if increases.len() > decreases.len() {
        decreases.to_vec()
    } else {
        increases.to_vec()
    };

    problematic.extend_from_slice(&invalid);
    problematic
}

// Validate the sequence after removing the item at an index
fn validate_sequence_with_removed(sequence: &[i32], index: usize) -> bool {
    // This could probably be done with some rule, and not actually removing the element...
    let mut temp_s = sequence.to_vec();
    temp_s.remove(index);

    validate_sequence(&temp_s)
}

// Count the number of occurrances of each diff type, classified as increasing, decreasing or invalid (flat or too big jump)
fn sort_increment_types(diffs: &[i32]) -> (Vec<DiffTuple>, Vec<DiffTuple>, Vec<DiffTuple>) {
    let (increases, decreases, invalid) = diffs.iter().enumerate().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut inc, mut dec, mut inv), (i, &value)| {
            if (1..4).contains(&value) {
                inc.push((i, value));
            } else if (-3..0).contains(&value) {
                dec.push((i, value));
            } else {
                inv.push((i, value));
            }
            (inc, dec, inv)
        },
    );

    (increases, decreases, invalid)
}

#[cfg(test)]
mod tests {
    use crate::{day2::solve, read_input_for_day};

    #[test]
    fn test_day_2() {
        let result =
            solve(&read_input_for_day(2).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(242, result[0]);
        assert_eq!(311, result[1]);
    }
}
