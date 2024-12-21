/// Solve the problem for day 18, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input_for_day;

    #[test]
    fn test_example_data() {
        let map = r"";

        let data: Vec<String> = map.lines().map(String::from).collect();

        let result = solve(&data).unwrap();
        assert_eq!(
            4635635210, result[0],
            "Result for part 1 example should be 4635635210 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_18() {
        let result =
            solve(&read_input_for_day(18).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(0, result[0]);
        assert_eq!(0, result[1]);
    }
}