/// Solve the problem for day one, given the provided data.
pub fn solve(data: &Vec<String>) -> Result<Vec<u32>, String> {
    let answer_part_1 = solve_part_1(data)?;
    //let answer_part_2 = solve_part_2(data)?;

    Ok(vec![answer_part_1, 0])
}

fn solve_part_1(data: &Vec<String>) -> Result<u32, String> {
    let mut first_list: Vec<u32> = Vec::with_capacity(data.len());
    let mut second_list: Vec<u32> = Vec::with_capacity(data.len());
    for s in data.iter() {
        let (first_val, second_val) = split_entry(s)?;
        first_list.push(first_val);
        second_list.push(second_val);
    }

    first_list.sort();
    second_list.sort();

    let mut result = 0;
    for (first, second) in first_list.iter().zip(second_list) {
        result += first.abs_diff(second);
    }

    Ok(result)
}

fn split_entry(s: &String) -> Result<(u32, u32), String> {
    let parts: Vec<&str> = s.split("   ").collect();

    if parts.len() != 2 {
        return Err(format!("Failed to split data entry {} properly.", s));
    }

    let first_value = parts[0]
        .parse::<u32>()
        .or(Err(format!("Failed to parse {} into u32.", parts[0])))?;

    let second_value = parts[1]
        .parse::<u32>()
        .or(Err(format!("Failed to parse {} into u32.", parts[1])))?;

    Ok((first_value, second_value))
}

fn solve_part_2(data: &Vec<String>) -> Result<u32, String> {
    Err("Day 1 part 2 not solved yet".to_string())
}
#[cfg(test)]
mod tests {
    use crate::{day1::solve, read_input_for_day};

    #[test]
    fn test_day_1() {
        // Wip
        let result =
            solve(&read_input_for_day(&1).expect("Expect the data file to be there.")).unwrap();
        assert_eq!(2367773, result[0]);
    }
}
