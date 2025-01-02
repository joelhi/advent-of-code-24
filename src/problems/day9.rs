use std::collections::HashSet;

/// Solve the problem for day nine, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let data = input_data.join("");
    let expanded_mem = expand_mem(&data);
    // Part 1
    let mut part_1_data = expanded_mem.clone();
    compact_mem(&mut part_1_data);

    let result_part_1 = checksum(&part_1_data);

    // Part 2
    let mut part_2_data = expanded_mem.clone();
    compact_mem_blocks(&mut part_2_data);

    let result_part_2 = checksum(&part_2_data);

    Ok(vec![result_part_1, result_part_2])
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

/// Compact the memory into a single contiguous block
fn compact_mem(expanded_mem: &mut [Option<u64>]) {
    let mut prev_empty = 0;
    let mut prev_last = expanded_mem.len();
    while let Some((last_id, last_val)) = expanded_mem[0..prev_last]
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, opt)| opt.map(|v| (i, v)))
    {
        if let Some(empty_id) = expanded_mem[prev_empty..last_id]
            .iter()
            .enumerate()
            .find(|(_, opt)| opt.is_none())
            .map(|(id, _)| id)
        {
            expanded_mem[last_id] = None;
            expanded_mem[prev_empty + empty_id] = Some(last_val);

            prev_empty = prev_empty + empty_id + 1;
            prev_last = last_id;
        } else {
            // No more free spots before the last valid element
            break;
        }
    }
}

/// Compact the memory as blocks.
fn compact_mem_blocks(expanded_mem: &mut [Option<u64>]) {
    let mut prev_data_index = expanded_mem.len();
    let mut processed_ids = HashSet::new();
    while let Some((data_index, data_len, val)) =
        find_next_data_block(prev_data_index, expanded_mem)
    {
        let mut prev_free_index = 0;
        while let Some((free_index, free_len)) = find_next_free_block(prev_free_index, expanded_mem)
        {
            if !processed_ids.contains(&val) && free_index < data_index && free_len >= data_len {
                for i in 0..data_len {
                    expanded_mem[data_index + i] = None;
                    expanded_mem[free_index + i] = Some(val);
                }
                processed_ids.insert(val);
                break;
            } else {
                // Update new location for triggering next search
                prev_free_index = free_index + free_len;
            }
        }
        prev_data_index = data_index;
    }
}

/// Find the next free block of memory. Returns a tuple with the start index and the length.
fn find_next_free_block(
    start_index: usize,
    expanded_mem: &[Option<u64>],
) -> Option<(usize, usize)> {
    if let Some(free_index) = expanded_mem[start_index..]
        .iter()
        .enumerate()
        .find(|(_, opt)| opt.is_none())
        .map(|(id, _)| id)
    {
        let index = start_index + free_index;
        let mut len = 1;
        while index + len < expanded_mem.len() - 1 && expanded_mem[index + len].is_none() {
            len += 1
        }

        Some((index, len))
    } else {
        None
    }
}

/// Find the next free block of memory. Returns a tuple with the start index and the length, and the value.
fn find_next_data_block(
    last_index: usize,
    expanded_mem: &[Option<u64>],
) -> Option<(usize, usize, u64)> {
    if let Some((index, value)) = expanded_mem[0..last_index]
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, opt)| opt.map(|v| (i, v)))
    {
        let mut len = 1;
        while len <= index && expanded_mem[index - len] == Some(value) {
            len += 1
        }

        Some((index - (len - 1), len, value))
    } else {
        None
    }
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
    use super::*;

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
        assert_eq!(
            2858, result[1],
            "Result for part 2 example should be 2858 but was {}",
            result[1]
        );
    }

    #[test]
    #[cfg(feature = "real_inputs")]
    fn test_day_9() {
        let result = solve(&crate::read_input_for_day(9).expect(
            "To run the tests for the real inputs the file has to be found in the inputs folder.",
        ))
        .unwrap();

        assert_eq!(6463499258318, result[0]);
        assert_eq!(6493634986625, result[1]);
    }
}
