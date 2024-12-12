use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use super::utils;

type Vec2u = (usize, usize);

/// Solve the problem for day twelve, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let result_part_1 = sat_bfs(input_data)?.iter().map(|(a, p)| a * p).sum();

    Ok(vec![result_part_1, 0])
}

/// Find all continuous regions using iterative, saturating bfs search.
/// Returns a tuple with area and perimeter of all found areas.
fn sat_bfs(map: &[String]) -> Result<Vec<(u64, u64)>, String> {
    // Start from first available node
    let num_rows = map.len();
    let num_cols = map[0].len();
    let mut visited = HashSet::with_capacity(num_rows * num_cols);
    let mut regions = Vec::new();
    while let Some((i, j)) = find_next_start(num_rows, num_cols, &visited) {
        let plant_type =
            utils::get_char(map, i, j).expect("Character should be valid for start node.");
        let mut area = 0;
        let mut perimeter = 0;

        // Perform bfs from the start node
        let mut queue = VecDeque::new();
        queue.push_back((i, j));
        while let Some(next) = queue.pop_front() {
            if !visited.insert(next) {
                continue;
            }
            area += 1;
            for (d_i, d_j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some(next) = utils::increment_2d_index(next.0, next.1, d_i, d_j, 1) {
                    let next_char = utils::get_char(map, next.0, next.1);
                    if Some(plant_type) == next_char {
                        if !visited.contains(&next) {
                            queue.push_back(next);
                        }
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        regions.push((area, perimeter));
    }

    Ok(regions)
}

/// Find next unvisited start node. If none available, return [`None`].
fn find_next_start(nuw_rows: usize, num_cols: usize, visited: &HashSet<Vec2u>) -> Option<Vec2u> {
    for i in 0..nuw_rows {
        for j in 0..num_cols {
            if !visited.contains(&(i, j)) {
                return Some((i, j));
            }
        }
    }
    None
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
