use std::collections::{HashMap, HashSet, VecDeque};

use crate::{increment_2d_index, Vec2u};

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

/// Solve the problem for day one, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let (start, _, walls) = parse_maze(input_data)?;

    let cheat_options = find_cheat_options(&start, &walls, 100);

    Ok(vec![cheat_options.len() as u64, 0])
}

/// Find possible cheating options
fn find_cheat_options(source: &Vec2u, walls: &HashSet<Vec2u>, min_length: usize) -> Vec<usize> {
    let start_dist_map = distance_map(source, walls, usize::MAX);
    let mut cheats = Vec::new();
    for (pos, dist) in start_dist_map.iter() {
        let options = distance_map(pos, &HashSet::new(), 2);
        cheats.extend(
            options
                .iter()
                .map(|(pos, step)| {
                    let cheat_end_dist = *start_dist_map.get(pos).unwrap_or(&(dist + step));
                    cheat_end_dist.saturating_sub(dist + step)
                })
                .filter(|val| *val >= min_length),
        );
    }

    cheats
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
    let mut visited = HashMap::new();
    queue.push_back((*source, 0));
    while let Some(next) = queue.pop_front() {
        if !(visited.contains_key(&next.0) || next.1 > max_dist) {
            visited.insert(next.0, next.1);
            queue.extend(step(&next, walls, &visited));
        }
    }

    visited
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

        let (start, _, walls) = parse_maze(&data).unwrap();

        let cheat_options = find_cheat_options(&start, &walls, 1);
        assert_eq!(
            44,
            cheat_options.len(),
            "Result for part 1 example should be 44 but was {}",
            cheat_options.len()
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
