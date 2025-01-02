use std::collections::{HashMap, HashSet};

use crate::{increment_2d_index, ortho_dir, Vec2i, Vec2u};

use super::utils;

/// Solve the problem for day six, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    // Find all obstacles
    let mut obstacles: HashSet<Vec2u> = input_data
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, char)| char == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let original_state = find_guard_pos_and_dir(input_data)?;
    let limits = (input_data.len(), input_data[0].len());

    // Part 1
    let all_states =
        solve_part_1(original_state, &obstacles, limits).expect("Part one should be solveable.");

    // Part 2
    let result_part_2 = solve_part_2(&all_states, &mut obstacles, limits);

    Ok(vec![all_states.keys().len() as u64, result_part_2])
}

/// Solve part 1
fn solve_part_1(
    original_state: (Vec2u, Vec2i),
    obstacles: &HashSet<Vec2u>,
    limits: Vec2u,
) -> Option<HashMap<Vec2u, Vec<Vec2i>>> {
    let mut current_state = original_state;
    let mut previous_states: HashMap<Vec2u, Vec<Vec2i>> = HashMap::new();
    previous_states.insert(original_state.0, vec![original_state.1]);
    while let Some(next_state) = update_state(current_state, obstacles, limits) {
        if !add_state(&next_state, &mut previous_states) {
            return None;
        }
        current_state = next_state;
    }

    // Find all unique positions
    Some(previous_states)
}

/// Add the state to the list if not a duplicate
fn add_state(state: &(Vec2u, Vec2i), previous_states: &mut HashMap<Vec2u, Vec<Vec2i>>) -> bool {
    let dirs = previous_states
        .entry(state.0)
        .or_insert(Vec::with_capacity(8));

    if !dirs.contains(&state.1) {
        dirs.push(state.1);
        true
    } else {
        false
    }
}

/// Solve part 2
fn solve_part_2(
    all_states: &HashMap<Vec2u, Vec<Vec2i>>,
    obstacles: &mut HashSet<Vec2u>,
    limits: Vec2u,
) -> u64 {
    // Put obstacle on all visited pos, check if path is broken
    let mut count = 0;
    for (&(i, j), dirs) in all_states.iter() {
        obstacles.insert((i, j));
        let (v_i, v_j) = dirs[0];
        let prev_pos = increment_2d_index(i, j, -v_i, -v_j, 1).unwrap();
        match solve_part_1((prev_pos, (v_i, v_j)), obstacles, limits) {
            Some(_) => (),
            None => {
                count += 1;
            }
        };
        obstacles.remove(&(i, j));
    }

    count
}

/// Update the position and direction based on the guards movement.
fn update_state(
    state: (Vec2u, Vec2i),
    obstacles: &HashSet<Vec2u>,
    limits: Vec2u,
) -> Option<(Vec2u, Vec2i)> {
    let ((i, j), (v_i, v_j)) = state;
    if let Some(pos) = utils::increment_2d_index(i, j, v_i, v_j, 1) {
        if obstacles.contains(&pos) {
            let v_perp = ortho_dir((v_i, v_j), true);
            return update_state(((i, j), v_perp), obstacles, limits);
        } else if pos.0 < limits.0 && pos.1 < limits.1 {
            return Some((pos, (v_i, v_j)));
        }
    }

    // no valid position
    None
}

/// Find the location and direction of travel for the guard in the data.
fn find_guard_pos_and_dir(input_data: &[String]) -> Result<(Vec2u, Vec2i), String> {
    for (i, s) in input_data.iter().enumerate() {
        if let Some(j) = s.find(">") {
            return Ok(((i, j), (0, 1)));
        } else if let Some(j) = s.find("^") {
            return Ok(((i, j), (-1, 0)));
        } else if let Some(j) = s.find("<") {
            return Ok(((i, j), (0, -1)));
        } else if let Some(j) = s.find("v") {
            return Ok(((i, j), (-1, 0)));
        }
    }
    Err("Not valid guard found.".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("....#.....").unwrap());
        data.push(String::from_str(".........#").unwrap());
        data.push(String::from_str("..........").unwrap());
        data.push(String::from_str("..#.......").unwrap());
        data.push(String::from_str(".......#..").unwrap());
        data.push(String::from_str("..........").unwrap());
        data.push(String::from_str(".#..^.....").unwrap());
        data.push(String::from_str("........#.").unwrap());
        data.push(String::from_str("#.........").unwrap());
        data.push(String::from_str("......#...").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            41, result[0],
            "Result for part 1 example should be 41 but was {}",
            result[0]
        );
        assert_eq!(
            6, result[1],
            "Result for part 2 example should be 6 but was {}",
            result[1]
        );
    }

    #[test]
    #[cfg(feature = "real_inputs")]
    fn test_day_6() {
        let result = solve(&crate::read_input_for_day(6).expect(
            "To run the tests for the real inputs the file has to be found in the inputs folder.",
        ))
        .unwrap();

        assert_eq!(5080, result[0]);
        assert_eq!(1919, result[1]);
    }
}
