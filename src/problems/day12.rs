use std::collections::{HashSet, VecDeque};

use super::utils;

const NEIGHBORS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

type Vec2u = (usize, usize);

/// Solve the problem for day twelve, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let result_part_1 = find_regions(input_data)?.iter().map(|(a, p)| a * p).sum();

    Ok(vec![result_part_1, 0])
}

/// Find all continuous regions using iterative, saturating bfs search.
/// Returns a tuple with area and perimeter of all found areas.
fn find_regions(map: &[String]) -> Result<Vec<(u64, u64)>, String> {
    // Start from first available node
    let num_rows = map.len();
    let num_cols = map[0].len();
    let mut visited = HashSet::with_capacity(num_rows * num_cols);
    let mut regions = Vec::new();
    for i in 0..num_rows {
        for j in 0..num_cols {
            if visited.contains(&(i, j)) {
                continue;
            }

            if let Some(plant_type) = utils::get_char(map, i, j) {
                let (area, perimeter) = bfs(i, j, plant_type, map, &mut visited);
                regions.push((area, perimeter));
            }
        }
    }

    Ok(regions)
}

/// Compute a bfs search to track the continous region with the certain char.
fn bfs(
    i: usize,
    j: usize,
    plant_type: char,
    map: &[String],
    visited: &mut HashSet<Vec2u>,
) -> (u64, u64) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    while let Some(next) = queue.pop_front() {
        if !visited.insert(next) {
            continue;
        }

        for &(d_i, d_j) in NEIGHBORS {
            if let Some(neighbour) = utils::increment_2d_index(next.0, next.1, d_i, d_j, 1) {
                if Some(plant_type) == utils::get_char(map, neighbour.0, neighbour.1) {
                    if !visited.contains(&neighbour) {
                        queue.push_back(neighbour);
                    }
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
        area += 1;
    }
    (area, perimeter)
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

        assert_eq!(1371306, result[0]);
        assert_eq!(0, result[1]);
    }
}
