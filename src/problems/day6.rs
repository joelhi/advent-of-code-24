use std::collections::HashSet;

use super::utils;

// Unsigned 2d coord
type Vec2u = (usize, usize);
// Signed 2d coord
type Vec2i = (isize, isize);

/// Solve the problem for day six, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u32>, String> {
    // Find all obstacles
    let obstacles: HashSet<Vec2u> = input_data
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, char)| char == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    // Part 1
    let result_part_1 = solve_part_1(
        find_guard_pos_and_dir(&input_data)?,
        &obstacles,
        (input_data.len(), input_data[0].len()),
    )?;

    Ok(vec![result_part_1, 0])
}

/// Solve part 1
fn solve_part_1(
    original_state: (Vec2u, Vec2i),
    obstacles: &HashSet<Vec2u>,
    limits: Vec2u,
) -> Result<u32, String> {
    let mut current_state = original_state;
    let mut unique_pos: HashSet<Vec2u> = HashSet::new();
    unique_pos.insert(original_state.0);
    while let Some(next_state) = update_state(current_state, obstacles, limits) {
        unique_pos.insert(next_state.0);
        current_state = next_state;
    }

    Ok(unique_pos.len() as u32)
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
            let (v_i, v_j) = update_dir((v_i, v_j))?;
            return update_state(((i, j), (v_i, v_j)), obstacles, limits);
        } else if pos.0 < limits.0 && pos.1 < limits.1 {
            return Some((pos, (v_i, v_j)));
        }
    }

    // no valid position
    None
}

/// Rotate the direction clockwise
fn update_dir(v: Vec2i) -> Option<Vec2i> {
    let (v_i, v_j) = v;
    if v_i < 0 {
        Some((0, 1))
    } else if v_i > 0 {
        Some((0, -1))
    } else if v_j < 0 {
        Some((-1, 0))
    } else if v_j > 0 {
        Some((1, 0))
    } else {
        None
    }
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
    use std::str::FromStr;

    use super::solve;
    use crate::read_input_for_day;

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
    }

    #[test]
    fn test_day_6() {
        let result =
            solve(&read_input_for_day(&6).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(5080, result[0]);
        assert_eq!(0, result[1]);
    }
}
