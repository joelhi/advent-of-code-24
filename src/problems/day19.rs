use std::collections::{HashMap, VecDeque};

/// Solve the problem for day 18, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let (designs, patterns) = parse_input(input_data);
    let solutions = match_patterns(&designs, &patterns);

    Ok(vec![
        solutions.iter().filter(|&s| *s > 0).count() as u64,
        solutions.iter().sum::<usize>() as u64,
    ])
}

/// Parse the input data into patterns and designs.
fn parse_input(input_data: &[String]) -> (Vec<&str>, Vec<&str>) {
    let patterns = input_data[0].split(',').map(|s| s.trim()).collect();
    let designs = input_data[2..].iter().map(|s| s.as_str()).collect();

    (designs, patterns)
}

/// Match the patterns to the designs. Return the ones that are solveable.
fn match_patterns(designs: &[&str], patterns: &[&str]) -> Vec<usize> {
    let mut solutions = Vec::new();
    for &design in designs {
        solutions.push(can_make(design, patterns));
    }

    solutions
}

/// Check if the design can be made from the available patterns.
fn can_make(design: &str, patterns: &[&str]) -> usize {
    let mut solutions_at = HashMap::new();

    for i in 0..design.len() {
        let index = design.len() - (i + 1);
        let num_solutions = solve_from_index(design, patterns, index, &solutions_at);
        solutions_at.insert(index, num_solutions);
    }

    *solutions_at.get(&0).unwrap_or(&0)
}

/// Find the number of solutions, starting from the specific index.
fn solve_from_index(
    design: &str,
    patterns: &[&str],
    index: usize,
    solutions_at: &HashMap<usize, usize>,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(index);

    let mut options = 0;
    while let Some(start) = queue.pop_back() {
        if start == design.len() || solutions_at.contains_key(&start) {
            options += solutions_at.get(&start).unwrap_or(&1);
            continue;
        }

        queue.extend(find_matches_at(design, start, patterns));
    }

    options
}

/// Find all the matches at the current start index
fn find_matches_at(design: &str, start: usize, patterns: &[&str]) -> Vec<usize> {
    let mut next_indices = Vec::new();
    for &pattern in patterns {
        if design[start..].starts_with(pattern) {
            next_indices.push(start + pattern.len());
        }
    }

    next_indices
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input_for_day;

    #[test]
    fn test_example_data() {
        let map = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let data: Vec<String> = map.lines().map(String::from).collect();

        let result = solve(&data).unwrap();
        assert_eq!(
            6, result[0],
            "Result for part 1 example should be 6 but was {}",
            result[0]
        );
        assert_eq!(
            16, result[1],
            "Result for part 2 example should be 16 but was {}",
            result[1]
        );
    }

    #[test]
    fn test_day_19() {
        let result =
            solve(&read_input_for_day(19).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(276, result[0]);
        assert_eq!(681226908011510, result[1]);
    }
}
