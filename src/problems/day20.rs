use std::collections::HashSet;

use crate::Vec2u;

/// Solve the problem for day one, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let (start, end, walls) = parse_maze(input_data)?;

    Ok(vec![0,0])
}

/// Parse start, end, and wall locations from the map.
fn parse_maze(input_data: &[String]) -> Result<(Vec2u, Vec2u, HashSet<Vec2u>), String> {
    let mut start = None;
    let mut end = None;
    let mut walls = HashSet::new();

    for (i, s) in input_data.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((i, j));
                }
                'S' => start = Some((i, j)),
                'E' => end = Some((i, j)),
                _ => {}
            }
        }
    }

    let start = start.ok_or("Failed to find start point")?;
    let end = end.ok_or("Failed to find start point")?;

    Ok((start, end, walls))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input_for_day;

    #[test]
    fn test_example_data() {
        let map = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let data: Vec<String> = map.lines().map(String::from).collect();

        let result = solve(&data).unwrap();
        assert_eq!(
            44, result[0],
            "Result for part 1 example should be 7036 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_20() {
        let result =
            solve(&read_input_for_day(20).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(0, result[0]);
        assert_eq!(0, result[1]);
    }
}
