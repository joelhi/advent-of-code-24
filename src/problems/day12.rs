/// Solve the problem for day twelve, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {

    Ok(vec![0, 0])
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
        data.push(String::from_str("RRRRIICCFF").unwrap());
        data.push(String::from_str("RRRRIICCCF").unwrap());
        data.push(String::from_str("VVRRRCCFFF").unwrap());
        data.push(String::from_str("VVRCCCJFFF").unwrap());
        data.push(String::from_str("VVVVCJJCFE").unwrap());
        data.push(String::from_str("VVIVCCJJEE").unwrap());
        data.push(String::from_str("VVIIICJJEE").unwrap());
        data.push(String::from_str("MIIIIIJJEE").unwrap());
        data.push(String::from_str("MIIISIJEEE").unwrap());
        data.push(String::from_str("MMMISSJEEE").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            1930, result[0],
            "Result for part 1 example should be 55312 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_12() {
        let result =
            solve(&read_input_for_day(&12).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(0, result[0]);
        assert_eq!(0, result[1]);
    }
}