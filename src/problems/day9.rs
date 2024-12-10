/// Solve the problem for day nine, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let data = input_data.join("");

    let mut expanded_mem = expand_mem(&data);
    compact_mem(&mut expanded_mem)?;

    let result_part_1 = checksum(&expanded_mem);

    Ok(vec![result_part_1, 0])
}

/// Expand the compact data into a full list of the memory
fn expand_mem(data: &str) -> Vec<Option<u64>> {
    let input_values: Vec<u64> = data
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|v| v as u64)
        .collect();

    let mut expanded = Vec::with_capacity(input_values.len() * 5);
    let mut is_space = false;
    for (id, &val) in input_values.iter().enumerate() {
        for _ in 0..val {
            if is_space {
                expanded.push(None);
            } else {
                expanded.push(Some(id as u64 / 2));
            }
        }
        is_space = !is_space;
    }

    expanded
}

// Compact the memory into a single contiguous block
fn compact_mem(expanded_memory: &mut [Option<u64>]) -> Result<(), String> {
    let mut prev_empty = 0;
    let mut prev_last = expanded_memory.len();
    loop {
        if let Some((last_id, last_val)) = expanded_memory[0..prev_last]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, opt)| opt.map(|v| (i, v)))
        {
            if let Some(empty_id) = expanded_memory[prev_empty..last_id]
                .iter()
                .enumerate()
                .find(|(_, opt)| opt.is_none())
                .map(|(id, _)| id)
            {
                expanded_memory[last_id] = None;
                expanded_memory[prev_empty + empty_id] = Some(last_val);

                prev_empty = prev_empty + empty_id + 1;
                prev_last = last_id;
            } else {
                // No more free spots before the last valid element
                break;
            }
        } else {
            // Could not find any more Nones
            return Err("Failed to get None. Data must have been corrupted.".to_owned());
        }
    }
    Ok(())
}

/// Compute the checksum for the memory
fn checksum(expanded_memory: &[Option<u64>]) -> u64 {
    expanded_memory
        .iter()
        .enumerate()
        .filter_map(|(i, val)| val.map(|v| i as u64 * v)) // Unwrap and compute in one step
        .sum::<u64>()
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
        data.push(String::from_str("2333133121414131402").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            1928, result[0],
            "Result for part 1 example should be 1928 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_9() {
        let result =
            solve(&read_input_for_day(&9).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(6463499258318, result[0]);
        assert_eq!(0, result[1]);
    }
}
