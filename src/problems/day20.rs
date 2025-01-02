use std::collections::{HashMap, HashSet, VecDeque};

use crate::{increment_2d_index, Vec2u};

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

/// Solve the problem for day one, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let (start, _, walls) = parse_maze(input_data)?;

    let cheat_options_part_1 = find_cheat_options(&start, &walls, 100, 2);
    let cheat_options_part_2 = find_cheat_options(&start, &walls, 100, 20);

    Ok(vec![
        cheat_options_part_1.len() as u64,
        cheat_options_part_2.len() as u64,
    ])
}

/// Find possible cheating options
fn find_cheat_options(
    source: &Vec2u,
    walls: &HashSet<Vec2u>,
    min_length: usize,
    cheat_steps: usize,
) -> Vec<usize> {
    // Distance map from start for each pos in track.
    let start_dist_map = distance_map(source, walls, usize::MAX);

    // Find possible cheats for each location along the track.
    start_dist_map
        .iter()
        .flat_map(|(&pos, &start_dist)| {
            let start_dist_map = &start_dist_map;
            distance_map(&pos, &HashSet::new(), cheat_steps)
                .into_iter()
                .filter_map(move |(reachable_pos, steps)| {
                    let end_dist = start_dist_map
                        .get(&reachable_pos)
                        .copied()
                        .unwrap_or(start_dist + steps);
                    let cheat_advantage = end_dist.saturating_sub(start_dist + steps);
                    (cheat_advantage >= min_length).then_some(cheat_advantage)
                })
        })
        .collect()
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

/// Trace path using bfs search
fn distance_map(source: &Vec2u, walls: &HashSet<Vec2u>, max_dist: usize) -> HashMap<Vec2u, usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    queue.push_back((*source, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if distances.contains_key(&pos) || dist > max_dist {
            continue;
        }

        distances.insert(pos, dist);
        queue.extend(step(&(pos, dist), walls, &distances));
    }

    distances
}

/// Compute valid steps from the current pos
fn step(
    pos: &(Vec2u, usize),
    walls: &HashSet<Vec2u>,
    visited: &HashMap<Vec2u, usize>,
) -> Vec<(Vec2u, usize)> {
    DIRECTIONS
        .iter()
        .filter_map(|&next_dir| {
            increment_2d_index(pos.0 .0, pos.0 .1, next_dir.0, next_dir.1, 1).and_then(|next| {
                if !(walls.contains(&next) || visited.contains_key(&next)) {
                    Some((next, pos.1 + 1))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let (start, _, walls) = parse_maze(&data).unwrap();

        let cheat_options_part_1 = find_cheat_options(&start, &walls, 1, 2);
        assert_eq!(
            44,
            cheat_options_part_1.len(),
            "Result for part 1 example should be 44 but was {}",
            cheat_options_part_1.len()
        );

        let cheat_options_part_2 = find_cheat_options(&start, &walls, 50, 20);
        assert_eq!(
            285,
            cheat_options_part_2.len(),
            "Result for part 2 example should be 285 but was {}",
            cheat_options_part_2.len()
        );
    }

    #[test]
    #[cfg(feature = "real_inputs")]
    fn test_day_20() {
        let result = solve(&crate::read_input_for_day(20).expect(
            "To run the tests for the real inputs the file has to be found in the inputs folder.",
        ))
        .unwrap();

        assert_eq!(1286, result[0]);
        assert_eq!(989316, result[1]);
    }
}
