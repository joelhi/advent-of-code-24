use std::collections::HashMap;

/// Solve the problem for day five, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u32>, String> {
    // Structure input data
    let (order_rules, sequences) = parse_input(input_data)?;

    // Solve part 1
    let result_part_1 = sequences
        .iter()
        .filter(|sequence| validate_sequence(sequence, &order_rules))
        .map(|sequence| sequence[sequence.len()/2])
        .sum();

    Ok(vec![result_part_1, 0])
}

// Validate a sequence given the order rules
fn validate_sequence(sequence: &[u32], order_rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (index, &page) in sequence.iter().enumerate() {
        if let Some(rules) = order_rules.get(&page) {
            // Check if any preceding page violates the rule
            if sequence[..index].iter().any(|&prev_page| rules.contains(&prev_page)) {
                return false;
            }
        }
    }
    true
}

// Parse the input into separate data structures for the order pairs and the sequences
fn parse_input(input_data: &[String]) -> Result<(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>), String> {
    let mut iter = input_data.split(|line| line.is_empty());

    // Parse the order pairs into a map
    let order_rules = iter
        .next()
        .unwrap_or(&[])
        .iter()
        .map(|line| parse_order_pair_from_str(line))
        .try_fold(HashMap::new(), |mut acc, result| {
            let (key, value) = result?;
            acc.entry(key).or_insert_with(Vec::new).push(value);
            Ok::<HashMap<_, Vec<_>>, String>(acc)
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

// Parse the rows containing an ordering into two u32s
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

// Parse a page sequence from a string of numbers
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
    fn test_day_5_example(){
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

        let input_data = data.iter().map(|&s| String::from_str(s).unwrap()).collect::<Vec<String>>();

        let result = solve(&input_data).unwrap();

        assert_eq!(143, result[0])
    }

    #[test]
    fn test_day_5() {
        let result =
            solve(&read_input_for_day(&5).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(0, result[0]);
        assert_eq!(0, result[1]);
    }
}


























