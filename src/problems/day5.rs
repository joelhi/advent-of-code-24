use std::collections::{HashMap, HashSet};

type ParsedInput = (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>);

/// Solve the problem for day five, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u32>, String> {
    // Structure input data
    let (order_rules, sequences) = parse_input(input_data)?;

    // Solve part 1
    let result_part_1 = sequences
        .iter()
        .filter(|sequence| validate_sequence(sequence, &order_rules))
        .map(|sequence| sequence[sequence.len() / 2])
        .sum();

    // Solve part 2
    let result_part_2 = sequences
        .iter()
        .filter(|sequence| !validate_sequence(sequence, &order_rules))
        .map(|sequence| fix_sequence_order(sequence, &order_rules))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|sequence| sequence[sequence.len() / 2])
        .sum();

    Ok(vec![result_part_1, result_part_2])
}

/// Validate a sequence given the order rules
fn validate_sequence(sequence: &[u32], order_rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (index, &page) in sequence.iter().enumerate() {
        if !sequence[..index]
            .iter()
            .all(|other_page| can_come_after(page, *other_page, order_rules))
        {
            return false;
        }
    }
    true
}

/// Check if the current page can come after the other other
fn can_come_after(
    current_page: u32,
    other_page: u32,
    order_rules: &HashMap<u32, Vec<u32>>,
) -> bool {
    if let Some(rules) = order_rules.get(&current_page) {
        !rules.contains(&other_page)
    } else {
        true
    }
}

/// Correct the sequence order based on the rules
fn fix_sequence_order(
    sequence: &[u32],
    order_rules: &HashMap<u32, Vec<u32>>,
) -> Result<Vec<u32>, String> {
    // Find valid locations for each page
    let mut new_sequence = Vec::with_capacity(sequence.len());
    let mut remaining_pages = HashSet::from_iter(sequence.iter().copied());
    while !remaining_pages.is_empty() {
        let next = find_next_valid(&remaining_pages, order_rules)?;
        new_sequence.push(next);
        remaining_pages.remove(&next);
    }

    Ok(new_sequence)
}

/// Return the next valid page in the original array
fn find_next_valid(
    remaining_pages: &HashSet<u32>,
    order_rules: &HashMap<u32, Vec<u32>>,
) -> Result<u32, String> {
    for &page in remaining_pages.iter() {
        if remaining_pages.iter().all(|&other_page| {
            other_page == page || can_come_after(other_page, page, order_rules)
        }) {
            // Adding this page will not invalidate any remaining page
            return Ok(page);
        }
    }

    Err("No valid order found".to_owned())
}

/// Parse the input into separate data structures for the order pairs and the sequences
fn parse_input(input_data: &[String]) -> Result<ParsedInput, String> {
    let mut iter = input_data.split(|line| line.is_empty());

    // Parse the order pairs into a map
    let order_rules = iter
        .next()
        .unwrap_or(&[])
        .iter()
        .map(|line| parse_order_pair_from_str(line))
        .try_fold(HashMap::new(), |mut acc: HashMap<u32, Vec<u32>>, result| {
            let (key, value) = result?;
            acc.entry(key).or_default().push(value);
            Ok::<_, String>(acc)
        })?;

    // Parse the page sequences into vecs
    let sequences = iter
        .next()
        .unwrap_or(&[])
        .iter()
        .map(|line| parse_page_sequence_from_str(line))
        .collect::<Result<Vec<_>, _>>()?;

    Ok((order_rules, sequences))
}

/// Parse the rows containing an ordering into two u32s
fn parse_order_pair_from_str(text: &str) -> Result<(u32, u32), String> {
    let parts = text.split("|").collect::<Vec<&str>>();

    if !parts.len() == 2 {
        return Err(format!("Failed to parse order pair {}.", text));
    }

    let first_value = parts[0]
        .parse::<u32>()
        .or(Err(format!("Failed to parse {} into u32.", parts[0])))?;

    let second_value = parts[1]
        .parse::<u32>()
        .or(Err(format!("Failed to parse {} into u32.", parts[1])))?;

    Ok((first_value, second_value))
}

/// Parse a page sequence from a string of numbers
fn parse_page_sequence_from_str(text: &str) -> Result<Vec<u32>, String> {
    text.split(",")
        .map(|s| {
            s.parse::<u32>()
                .or(Err(format!("Failed to parse {} into u32.", s)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::solve;
    use crate::read_input_for_day;

    #[test]
    fn test_day_5_example() {
        let mut data = Vec::new();

        data.push("47|53");
        data.push("97|13");
        data.push("97|61");
        data.push("97|47");
        data.push("75|29");
        data.push("61|13");
        data.push("75|53");
        data.push("29|13");
        data.push("97|29");
        data.push("53|29");
        data.push("61|53");
        data.push("97|53");
        data.push("61|29");
        data.push("47|13");
        data.push("75|47");
        data.push("97|75");
        data.push("47|61");
        data.push("75|61");
        data.push("47|29");
        data.push("75|13");
        data.push("53|13");
        data.push("");
        data.push("75,47,61,53,29");
        data.push("97,61,53,29,13");
        data.push("75,29,13");
        data.push("75,97,47,61,53");
        data.push("61,13,29");
        data.push("97,13,75,29,47");

        let input_data = data
            .iter()
            .map(|&s| String::from_str(s).unwrap())
            .collect::<Vec<String>>();

        let result = solve(&input_data).unwrap();

        assert_eq!(
            143, result[0],
            "Example result for part 1 should be 143, but was {}",
            result[0]
        );
        assert_eq!(
            123, result[1],
            "Example result for part 2 should be 123, but was {}",
            result[1]
        );
    }

    #[test]
    fn test_day_5() {
        let result =
            solve(&read_input_for_day(&5).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(5374, result[0]);
        assert_eq!(4260, result[1]);
    }
}
