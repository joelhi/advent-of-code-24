use std::collections::HashSet;

use crate::increment_2d_index;

type Vec2u = (usize, usize);
type Vec2i = (isize, isize);
type MapData = (HashSet<Vec2u>, HashSet<Vec2u>);

/// Solve the problem for day 15, given the provided data.
pub fn solve(input_data: &[String]) -> Result<Vec<u64>, String> {
    let (mut map_data, start, commands) = parse_input(input_data)?;

    // Part 1
    execute_commands(&mut map_data, start, commands)?;

    let result_part_1 = map_data
        .1
        .iter()
        .map(|pos| 100 * pos.0 + pos.1)
        .sum::<usize>() as u64;

    Ok(vec![result_part_1, 0])
}

/// Execute a sequence of commands and move the boxes.
fn execute_commands(
    map_data: &mut MapData,
    start: Vec2u,
    commands: &[String],
) -> Result<(), String> {
    let mut pos = start.clone();
    for sequence in commands {
        for command in sequence.chars() {
            execute_command(map_data, &mut pos, &command)?;
        }
    }

    Ok(())
}

/// Execute a command, and update the current position and boxes if applicable.
fn execute_command(map_data: &mut MapData, pos: &mut Vec2u, command: &char) -> Result<(), String> {
    let (walls, boxes) = map_data;
    let step = step_from_command(command)?;
    let new_pos = increment_2d_index(pos.0, pos.1, step.0, step.1, 1).ok_or(format!(
        "Failed to increment coord {:?} with {:?}",
        pos, step
    ))?;

    if walls.contains(&new_pos) {
        return Ok(());
    } else if boxes.contains(&new_pos) {
        if move_box(boxes, walls, &new_pos, &step)?{
            boxes.remove(&new_pos);
            *pos = new_pos;
        }
    } else {
        *pos = new_pos;
    }

    Ok(())
}

/// Move box to next position if possible
fn move_box(
    boxes: &mut HashSet<Vec2u>,
    walls: &HashSet<Vec2u>,
    pos: &Vec2u,
    step: &Vec2i,
) -> Result<bool, String> {
    let next_pos = increment_2d_index(pos.0, pos.1, step.0, step.1, 1).ok_or(format!(
        "Failed to increment coord {:?} with {:?}",
        pos, step
    ))?;

    if boxes.contains(&next_pos) {
        move_box(boxes, walls, &next_pos, step)
    } else if walls.contains(&next_pos) {
        Ok(false)
    } else {
        boxes.insert(next_pos);
        Ok(true)
    }
}

/// Get the step vector based on the command char.
fn step_from_command(command: &char) -> Result<Vec2i, String> {
    match command {
        '<' => Ok((0, -1)),
        '^' => Ok((-1, 0)),
        'v' => Ok((1, 0)),
        '>' => Ok((0, 1)),
        _ => Err(format!("Invalid command character {}", command)),
    }
}

/// Parse the input data
fn parse_input(input_data: &[String]) -> Result<(MapData, Vec2u, &[String]), String> {
    let mut split = input_data.split(|line| line.is_empty());

    let (map_data, start) = parse_input_map(
        split
            .next()
            .ok_or("Failed to parse map from input".to_owned())?,
    );

    let commands = split
        .next()
        .ok_or("Failed to parse map from input".to_owned())?;

    Ok((map_data, start, commands))
}

/// Parse the locations of all boxes and walls into two hash sets (walls, boxes).
fn parse_input_map(map: &[String]) -> (MapData, Vec2u) {
    let mut wall_pos = HashSet::new();
    let mut box_pos = HashSet::new();
    let mut start = (1, 1);
    for (i, line) in map.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    wall_pos.insert((i, j));
                }
                'O' => {
                    box_pos.insert((i, j));
                }
                '@' => {
                    start = (i, j);
                }
                _ => (),
            }
        }
    }

    ((wall_pos, box_pos), start)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::read_input_for_day;

    #[test]
    fn test_small_example_data() {
        let mut data = Vec::new();

        // Example data
        data.push(String::from_str("########").unwrap());
        data.push(String::from_str("#..O.O.#").unwrap());
        data.push(String::from_str("##@.O..#").unwrap());
        data.push(String::from_str("#...O..#").unwrap());
        data.push(String::from_str("#.#.O..#").unwrap());
        data.push(String::from_str("#...O..#").unwrap());
        data.push(String::from_str("#......#").unwrap());
        data.push(String::from_str("########").unwrap());
        data.push(String::from_str("").unwrap());
        data.push(String::from_str("<^^>>>vv<v>>v<<").unwrap());

        let result = solve(&data).unwrap();
        assert_eq!(
            2028, result[0],
            "Result for part 1 example should be 2028 but was {}",
            result[0]
        );
    }

    #[test]
    fn test_day_15() {
        let result =
            solve(&read_input_for_day(15).expect("Expect the data file to be there.")).unwrap();

        assert_eq!(1505963, result[0]);
        assert_eq!(0, result[1]);
    }
}
