use std::collections::HashSet;

use crate::Vec2u;

use super::utils;

/// Solve the problem for day ten, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let trail_heads = find_trailheads(input_data);

    let mut options = 0;
    let mut unique_options = 0;

    for head in trail_heads {
        let mut reached = Vec::new();
        step(0, head, &mut reached, input_data);

        options += reached.len() as u64;
        unique_options += reached.iter().collect::<HashSet<_>>().len() as u64;
    }

    Ok(vec![unique_options, options])
}

/// Find all the trailheads in the map.
fn find_trailheads(input_data: &[String]) -> Vec<Vec2u> {
    let mut trailheads = Vec::new();
    for (i, s) in input_data.iter().enumerate() {
        trailheads.extend(
            s.chars()
                .enumerate()
                .filter(|(_, c)| *c == '0')
                .map(move |(j, _)| (i, j)),
        );
    }

    trailheads
}

/// Compute step from current position
fn step(current_level: usize, current_pos: Vec2u, reached: &mut Vec<Vec2u>, map: &[String]) {
    if current_level == 9 {
        // Reached the end of a trail
        reached.push(current_pos);
    }

    // Find next steps
    for step in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        find_valid_step(&current_pos, current_level + 1, step, reached, map);
    }
}

// If a next valid step can be found, trigger it
fn find_valid_step(
    pos: &Vec2u,
    target_level: usize,
    increment: (isize, isize),
    reached: &mut Vec<Vec2u>,
    map: &[String],
) {
    let (i, j) = pos;
    if let Some((i, j)) = utils::increment_2d_index(*i, *j, increment.0, increment.1, 1) {
        if let Some(val) = utils::get_char(map, i, j) {
            if val.to_digit(10) == Some(target_level as u32) {
                step(target_level, (i, j), reached, map);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("89010123").unwrap());
        data.push(String::from_str("78121874").unwrap());
        data.push(String::from_str("87430965").unwrap());
        data.push(String::from_str("96549874").unwrap());
        data.push(String::from_str("45678903").unwrap());
        data.push(String::from_str("32019012").unwrap());
        data.push(String::from_str("01329801").unwrap());
        data.push(String::from_str("10456732").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            36, result[0],
            "Result for part 1 example should be 36 but was {}",
            result[0]
        );
        assert_eq!(
            81, result[1],
            "Result for part 1 example should be 81 but was {}",
            result[1]
        );
    }

    #[test]
    #[cfg(feature = "real_inputs")]
    fn test_day_10() {
        let result = solve(&crate::read_input_for_day(10).expect(
            "To run the tests for the real inputs the file has to be found in the inputs folder.",
        ))
        .unwrap();

        assert_eq!(746, result[0]);
        assert_eq!(1541, result[1]);
    }
}
