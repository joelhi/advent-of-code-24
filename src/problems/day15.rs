/// Solve the problem for day 15, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::read_input_for_day;

    #[test]
    fn test_small_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("########").unwrap());
        data.push(String::from_str("#..O.O.#").unwrap());
        data.push(String::from_str("##@.O..#").unwrap());
        data.push(String::from_str("#...O..#").unwrap());
        data.push(String::from_str("#.#.O..#").unwrap());
        data.push(String::from_str("#...O..#").unwrap());
        data.push(String::from_str("#......#").unwrap());
        data.push(String::from_str("########").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("<^^>>>vv<v>>v<<").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            2028, result[0],
            "Result for part 1 example should be 2028 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_14() {
        let result =
            solve(&read_input_for_day(15).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(232253028, result[0]);
        assert_eq!(8179, result[1]);
    }
}