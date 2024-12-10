use std::collections::{HashMap, HashSet};

use super::utils;

const NON_ANTENNA_SYMBOLS: &[char] = &['.', '#'];

type Vec2u = (usize, usize);

/// Solve the problem for day eight, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let antenna_locations = parse_antenna_locations(input_data);

    let unique_part_1 = find_unique_antinodes(
        &antenna_locations,
        (input_data.len(), input_data[0].len()),
        true,
    );

    let unique_part_2 = find_unique_antinodes(
        &antenna_locations,
        (input_data.len(), input_data[0].len()),
        false,
    );

    Ok(vec![unique_part_1.len() as u64, unique_part_2.len() as u64])
}

/// Parse all the locations for each type of antenna
fn parse_antenna_locations(input_data: &[String]) -> HashMap<char, Vec<Vec2u>> {
    let mut locations = HashMap::new();
    for (i, s) in input_data.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if !NON_ANTENNA_SYMBOLS.contains(&c) {
                locations.entry(c).or_insert_with(Vec::new).push((i, j));
            }
        }
    }

    locations
}

/// Find and count the unique anti nodes.
fn find_unique_antinodes(
    antenna_locations: &HashMap<char, Vec<Vec2u>>,
    bounds: Vec2u,
    single_pass: bool,
) -> HashSet<Vec2u> {
    let mut unique_pos = HashSet::new();
    for (_, locations) in antenna_locations.iter() {
        for i in 0..locations.len() {
            if !single_pass {
                unique_pos.insert(locations[i]);
            }
            for j in 0..locations.len() {
                unique_pos.extend(compute_all_antinodes(
                    locations[i],
                    locations[j],
                    bounds,
                    single_pass,
                ));
            }
        }
    }

    unique_pos
}

/// Check if antinode position is in bounds
fn in_bounds(node: Vec2u, bounds: &Vec2u) -> bool {
    node.0 < bounds.0 && node.1 < bounds.1
}

// Compute all the antinodes
fn compute_all_antinodes(
    first_antenna: Vec2u,
    second_antenna: Vec2u,
    bounds: Vec2u,
    single_pass: bool,
) -> Vec<Vec2u> {
    let mut all_nodes = Vec::new();
    let mut factor = 1;
    loop {
        let new_nodes = compute_antinodes(first_antenna, second_antenna, factor);
        if !add_nodes(new_nodes, &mut all_nodes, &bounds) || single_pass {
            break;
        }
        factor += 1;
    }

    all_nodes
}

// Add the valid nodes to the list. Return false if no nodes added.
fn add_nodes(new_nodes: [Option<Vec2u>; 2], all_nodes: &mut Vec<Vec2u>, bounds: &Vec2u) -> bool {
    let mut node_added = false;
    for &node in &new_nodes {
        if let Some(valid_node) = node {
            if in_bounds(valid_node, bounds) {
                all_nodes.push(valid_node);
                node_added = true;
            }
        }
    }
    node_added
}

// Compute the antinodes for two antenna locations
fn compute_antinodes(
    first_antenna: Vec2u,
    second_antenna: Vec2u,
    factor: usize,
) -> [Option<Vec2u>; 2] {
    if first_antenna == second_antenna {
        return [None, None];
    }

    let (diff_i, diff_j) = utils::difference_2i(first_antenna, second_antenna)
        .expect("Difference between nodes should be valid.");

    let first_node =
        utils::increment_2d_index(first_antenna.0, first_antenna.1, -diff_i, -diff_j, factor);
    let second_node =
        utils::increment_2d_index(second_antenna.0, second_antenna.1, diff_i, diff_j, factor);

    [first_node, second_node]
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
        data.push(String::from_str("............").unwrap());
        data.push(String::from_str("........0...").unwrap());
        data.push(String::from_str(".....0......").unwrap());
        data.push(String::from_str(".......0....").unwrap());
        data.push(String::from_str("....0.......").unwrap());
        data.push(String::from_str("......A.....").unwrap());
        data.push(String::from_str("............").unwrap());
        data.push(String::from_str("............").unwrap());
        data.push(String::from_str("........A...").unwrap());
        data.push(String::from_str(".........A..").unwrap());
        data.push(String::from_str("............").unwrap());
        data.push(String::from_str("............").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            14, result[0],
            "Result for part 1 example should be 14 but was {}",
            result[0]
        );

        assert_eq!(
            34, result[1],
            "Result for part 2 example should be 34 but was {}",
            result[1]
        );
    }

    #[test]
    fn test_day_8() {
        let result =
            solve(&read_input_for_day(&8).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(320, result[0]);
        assert_eq!(1157, result[1]);
    }
}
