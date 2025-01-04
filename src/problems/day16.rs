use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

use crate::{increment_2d_index, ortho_dir, Vec2i, Vec2u};

/// Struct to store the state of each path tracker
struct State(Vec2u, Vec2i, u64, Vec<Vec2u>);

/// Cost of an orthogonal turn
const ORTHOGONAL_COST: u64 = 1000;

/// Solve the problem for day 16, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let (start, end, walls) = parse_maze(input_data)?;
    let (result_part_1, result_part_2) = solve_parts(start, end, &walls);

    Ok(vec![result_part_1, result_part_2])
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

/// Solve part 1
fn solve_parts(start: Vec2u, end: Vec2u, walls: &HashSet<Vec2u>) -> (u64, u64) {
    let initial_state = State(start, (0, 1), 0, vec![start]);

    let mut queue = VecDeque::new();
    queue.push_back(initial_state);

    let mut paths = Vec::new();
    let mut visited = HashMap::new();
    while let Some(next) = queue.pop_front() {
        if next.0 == end {
            paths.push((next.3, next.2));
            continue;
        }
        if should_check_tile(&visited, &next.0, next.2) {
            visited.insert(next.0, next.2);
            queue.extend(step(&next, walls, &visited));
        }
    }

    analyse_paths(&paths)
}

/// Analyse the paths to find the minimum cost and all visited tiles.
fn analyse_paths(paths: &[(Vec<Vec2u>, u64)]) -> (u64, u64) {
    let min_cost = paths.iter().map(|(_, cost)| *cost).min().unwrap_or(0);

    let all_coord: HashSet<Vec2u> = paths
        .iter()
        .filter(|(_, cost)| *cost == min_cost)
        .map(|(path, _)| path)
        .flat_map(|path| path.iter())
        .copied()
        .collect();

    (min_cost, all_coord.len() as u64)
}

/// Check if we should stop at a tile. Stop if we already visited with a lower cost.
fn should_check_tile(visited: &HashMap<Vec2u, u64>, pos: &Vec2u, cost: u64) -> bool {
    if let Some(prev_cost) = visited.get(pos) {
        cost <= *prev_cost + 1000
    } else {
        true
    }
}

fn step(state: &State, walls: &HashSet<Vec2u>, visited: &HashMap<Vec2u, u64>) -> Vec<State> {
    let directions = [state.1, ortho_dir(state.1, true), ortho_dir(state.1, false)];

    directions
        .iter()
        .enumerate()
        .filter_map(|(i, &next_dir)| {
            increment_2d_index(state.0 .0, state.0 .1, next_dir.0, next_dir.1, 1).and_then(|next| {
                let cost = state.2 + 1 + if i == 0 { 0 } else { ORTHOGONAL_COST };
                if is_tile_valid(walls, visited, &next, cost) {
                    let mut path = state.3.clone();
                    path.push(next);
                    Some(State(next, next_dir, cost, path))
                } else {
                    None
                }
            })
        })
        .collect()
}

/// Check if a tile is valid for stepping into.
fn is_tile_valid(
    walls: &HashSet<Vec2u>,
    visited: &HashMap<Vec2u, u64>,
    tile: &Vec2u,
    cost: u64,
) -> bool {
    !walls.contains(tile) && should_check_tile(visited, tile, cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data() {
        let map = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let data: Vec<String> = map.lines().map(String::from).collect();

        let result = solve(&data).unwrap();
        assert_eq!(
            7036, result[0],
            "Result for part 1 example should be 7036 but was {}",
            result[0]
        );
        assert_eq!(
            45, result[1],
            "Result for part 1 example should be 45 but was {}",
            result[1]
        );
    }

    #[test]
    fn test_alt_example_data() {
        let map = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let data: Vec<String> = map.lines().map(String::from).collect();

        let result = solve(&data).unwrap();
        assert_eq!(
            11048, result[0],
            "Result for part 1 example should be 7036 but was {}",
            result[0]
        );
        assert_eq!(
            64, result[1],
            "Result for part 1 example should be 64 but was {}",
            result[1]
        );
    }

    #[test]
    #[cfg(feature = "real_inputs")]
    fn test_day_16() {
        let result = solve(&crate::read_input_for_day(16).expect(
            "To run the tests for the real inputs the file has to be found in the inputs folder.",
        ))
        .unwrap();

        assert_eq!(72400, result[0]);
        assert_eq!(435, result[1]);
    }
}
