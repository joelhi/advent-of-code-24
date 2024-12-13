use std::collections::{HashMap, HashSet, VecDeque};

use crate::increment_2d_index;

use super::utils;

const NEIGHBOURS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

type Vec2u = (usize, usize);

/// Solve the problem for day twelve, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let results = find_regions(input_data)?;

    let result_part_1 = results.iter().map(|(a, p, _)| a * p).sum();
    let result_part_2 = results.iter().map(|(a, _, s)| a * s).sum();

    Ok(vec![result_part_1, result_part_2])
}

/// Find all continuous regions using iterative, saturating bfs search.
/// Returns a tuple with area and perimeter data of all found areas.
fn find_regions(map: &[String]) -> Result<Vec<(u64, u64, u64)>, String> {
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
                regions.push(bfs((i, j), plant_type, map, &mut visited));
            }
        }
    }

    Ok(regions)
}

/// Compute a bfs search to track the continuous region with the certain char.
fn bfs(
    pos: Vec2u,
    plant_type: char,
    map: &[String],
    visited: &mut HashSet<Vec2u>,
) -> (u64, u64, u64) {
    let mut area = 0;
    let mut perimeters = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    while let Some(next) = queue.pop_front() {
        if visited.insert(next) {
            for &(d_i, d_j) in NEIGHBOURS {
                if let Some(neighbour) = utils::increment_2d_index(next.0, next.1, d_i, d_j, 1) {
                    if Some(plant_type) == utils::get_char(map, neighbour.0, neighbour.1) {
                        if !visited.contains(&neighbour) {
                            queue.push_back(neighbour);
                        }
                    } else {
                        perimeters
                            .entry(next)
                            .or_insert_with(HashSet::new)
                            .insert((d_i, d_j));
                    }
                } else {
                    perimeters
                        .entry(next)
                        .or_insert_with(HashSet::new)
                        .insert((d_i, d_j));
                }
            }
            area += 1;
        }
    }
    let (perimeter_length, num_perimeter_sides) = analyze_perimeters(&mut perimeters);
    (area, perimeter_length, num_perimeter_sides)
}

/// Analyze the recorded perimeter to find the total length and the number of sides
fn analyze_perimeters(perimeters: &mut HashMap<Vec2u, HashSet<(isize, isize)>>) -> (u64, u64) {
    let length = perimeters
        .iter()
        .map(|(_, perims)| perims.len() as u64)
        .sum();

    let mut sides = 0;
    let mut logged_perimeters = HashSet::new();
    for ((i, j), p) in perimeters.iter() {
        for (d_i, d_j) in p.iter() {
            let perim_state = (*i, *j, *d_i, *d_j);
            if logged_perimeters.insert(perim_state) {
                sides += 1;
                trace_side(perim_state, perimeters, &mut logged_perimeters, false);
                trace_side(perim_state, perimeters, &mut logged_perimeters, true);
            }
        }
    }

    (length, sides)
}

/// Trace a contiguous set of perimeter blocks.
fn trace_side(
    state: (usize, usize, isize, isize),
    perimeters: &HashMap<Vec2u, HashSet<(isize, isize)>>,
    logged_perimeters: &mut HashSet<(usize, usize, isize, isize)>,
    reverse: bool,
) {
    let (mut i, mut j, d_i, d_j) = state;
    let (step_i, step_j) = if reverse { (-d_j, -d_i) } else { (d_j, d_i) };

    loop {
        if let Some((next_i, next_j)) = increment_2d_index(i, j, step_i, step_j, 1) {
            if let Some(neighbor_perimeters) = perimeters.get(&(next_i, next_j).into()) {
                if neighbor_perimeters.contains(&(d_i, d_j)) {
                    logged_perimeters.insert((next_i, next_j, d_i, d_j));
                    i = next_i;
                    j = next_j;
                    continue;
                }
            }
        }
        break;
    }
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
            "Result for part 1 example should be 1930 but was {}",
            result[0]
        );
        assert_eq!(
            1206, result[1],
            "Result for part 2 example should be 1206 but was {}",
            result[1]
        );
    }

    #[test]
    fn test_day_12() {
        let result =
            solve(&read_input_for_day(12).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(1371306, result[0]);
        assert_eq!(805880, result[1]);
    }
}
