use std::collections::{HashSet, VecDeque};

use crate::{increment_2d_index, parse_pair_from_str};

type Vec2u = (usize, usize);

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

const MAP_SIZE: Vec2u = (71, 71);

/// Solve the problem for day 18, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let result_part_1 = solve_for_size(input_data, 0, 1024, MAP_SIZE)?;
    let result_part_2 = find_unsolveable_config(input_data, MAP_SIZE)?;
    Ok(vec![result_part_1, result_part_2.0 as u64, result_part_2.1 as u64])
}

/// Solve the problem for day 18, given the provided data.
pub fn solve_for_size(
    input_data: &[String],
    start: usize,
    end: usize,
    map_size: Vec2u,
) -> Result<u64, String> {
    let bytes = parse_bytes(input_data, start, end)?;

    let result = trace((0, 0), map_size, &bytes)?;

    Ok(result as u64)
}

/// Find unsolveable configuration of bytes, starting from part 1 and adding one at a time.
fn find_unsolveable_config(input_data: &[String], map_size: Vec2u)->Result<Vec2u, String>{
    let mut bytes = parse_bytes(input_data, 0, 1024)?;

    for i in 1024..input_data.len(){
        let next_byte = add_byte(input_data, &mut bytes, i)?;
        let result = trace((0,0), map_size, &bytes);

        if result.is_err(){
            return Ok(next_byte);
        }
    }

    Err("Failed to find unsolveable solution.".to_owned())
}

/// Parse the bytes from the input, based on start and end index
fn parse_bytes(input_data: &[String], start: usize, end: usize) -> Result<HashSet<Vec2u>, String> {
    input_data[start..end]
        .iter()
        .map(|s| parse_pair_from_str(s, ","))
        .collect()
}

/// Add an additional byte from the array. Returns the location of the added byte.
fn add_byte(input_data: &[String], bytes: &mut HashSet<Vec2u>, index: usize)->Result<Vec2u, String>{
    let byte = parse_pair_from_str(&input_data[index], ",")?;
    bytes.insert(byte);
    Ok(byte)
}

/// Trace path using bfs search
fn trace(start: Vec2u, map_size: Vec2u, bytes: &HashSet<Vec2u>) -> Result<usize, String> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let end = (map_size.0 - 1, map_size.1 - 1);
    let mut visited = HashSet::new();
    while let Some(next) = queue.pop_front() {
        if next.0 == end {
            return Ok(next.1);
        }
        if visited.insert(next.0) {
            queue.extend(step(&next, bytes, &visited, &map_size));
        }
    }

    Err("Process never reached the end.".to_owned())
}

/// Compute valid steps from the current pos
fn step(
    pos: &(Vec2u, usize),
    bytes: &HashSet<Vec2u>,
    visited: &HashSet<Vec2u>,
    map_size: &Vec2u,
) -> Vec<(Vec2u, usize)> {
    DIRECTIONS
        .iter()
        .filter_map(|&next_dir| {
            increment_2d_index(pos.0 .0, pos.0 .1, next_dir.0, next_dir.1, 1).filter(|next| {
                !bytes.contains(next)
                    && !visited.contains(next)
                    && next.0 < map_size.0
                    && next.1 < map_size.1
            })
        })
        .map(|p| (p, pos.1 + 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input_for_day;

    #[test]
    fn test_example_data() {
        let map = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        let data: Vec<String> = map.lines().map(String::from).collect();

        let result = solve_for_size(&data, 0, 12, (7,7)).unwrap();
        assert_eq!(
            22, result,
            "Result for part 1 example should be 22 but was {}",
            result
        );
    }

    #[test]
    fn test_day_18() {
        let result =
            solve(&read_input_for_day(18).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(416, result[0]);
        assert_eq!(50, result[1]);
        assert_eq!(23, result[2]);
    }
}
