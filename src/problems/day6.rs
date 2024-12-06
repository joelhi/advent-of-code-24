/// Solve the problem for day six, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u32>, String> {
    Err("Not Implemented".to_owned())
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::read_input_for_day;

    #[test]
    fn test_day_6() {
        let result =
            solve(&read_input_for_day(&6).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(0, result[0]);
        assert_eq!(0, result[1]);
    }

}