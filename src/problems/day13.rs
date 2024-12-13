/// Solve the problem for day thriteen, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {

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
        data.push(String::from_str("Button A: X+94, Y+34").unwrap());
        data.push(String::from_str("Button B: X+22, Y+67").unwrap());
        data.push(String::from_str("Prize: X=8400, Y=5400").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("Button A: X+26, Y+66").unwrap());
        data.push(String::from_str("Button B: X+67, Y+21").unwrap());
        data.push(String::from_str("Prize: X=12748, Y=12176").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("Button A: X+17, Y+86").unwrap());
        data.push(String::from_str("Button B: X+84, Y+37").unwrap());
        data.push(String::from_str("Prize: X=7870, Y=6450").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("Button A: X+69, Y+23").unwrap());
        data.push(String::from_str("Button B: X+27, Y+71").unwrap());
        data.push(String::from_str("Prize: X=18641, Y=10279").unwrap());
        data.push(String::from_str("").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            480, result[0],
            "Result for part 1 example should be 480 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_13() {
        let result =
            solve(&read_input_for_day(13).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(0, result[0]);
        assert_eq!(0, result[1]);
    }
}